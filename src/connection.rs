use std::sync::Arc;
use std::sync::mpsc;
use std::{str, u32};
use std::convert::TryInto;
use futures::executor::block_on;
use std::{error::Error, net::ToSocketAddrs, io as stdio};
use tokio_rustls::{TlsConnector, rustls::ClientConfig, webpki::DNSNameRef, client::TlsStream};
use tokio::{net::TcpStream, io::AsyncWriteExt, io::AsyncReadExt};

use crate::config::{CONFIG, EppClientConnection};
use crate::error;
use crate::epp::request::EppRequest;

struct EppConnection {
    registry: String,
    credentials: (String, String),
    stream: TlsStream<TcpStream>,
    pub greeting: String,
}

pub struct EppClient {
    connection: EppConnection,
}

impl EppConnection {
    pub async fn new(
        registry: String,
        credentials: (String, String),
        mut stream: TlsStream<TcpStream>) -> Result<EppConnection, Box<dyn Error>> {
        let mut buf = vec![0u8; 4096];
        stream.read(&mut buf).await?;
        let greeting = str::from_utf8(&buf)?.to_string();

        Ok(EppConnection {
            registry: registry,
            credentials: credentials,
            stream: stream,
            greeting: greeting
        })
    }

    async fn write(&mut self, buf: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        self.stream.write_all(buf).await?;
        Ok(())
    }

    pub async fn send_epp_request(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        let len = content.len();

        let buf_size = len + 4;
        let mut buf: Vec<u8> = vec![0u8; buf_size];

        let len_u32: [u8; 4] = u32::to_be_bytes(len.try_into()?);

        buf[..4].clone_from_slice(&len_u32);
        buf[4..].clone_from_slice(&content.as_bytes());

        self.write(&buf).await
    }

    async fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = vec![0u8; 4096];
        self.stream.read(&mut buf).await?;
        Ok(buf)
    }

    async fn read_epp_response(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).await?;

        let buf_size :usize = u32::from_be_bytes(buf).try_into()?;

        println!("Response buffer size: {}", buf_size);

        let mut buf = vec![0u8; buf_size - 4];

        self.stream.read(&mut buf).await?;

        Ok(buf)
    }

    pub async fn get_epp_response(&mut self) -> Result<String, Box<dyn Error>> {
        let contents = self.read().await?;

        let response = str::from_utf8(&contents)?.to_string();

        Ok(response)
    }

    pub async fn transact(&mut self, content: &str) -> Result<String, Box<dyn Error>> {
        let content = format!("{}\r\n\r\n", content);

        self.send_epp_request(&content).await?;
        self.get_epp_response().await
    }

    async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Closing ...");

        self.stream.shutdown().await?;
        Ok(())
    }
}

impl Drop for EppConnection {
    fn drop(&mut self) {
        block_on(self.close());
    }
}

impl EppClient {
    pub async fn transact(&mut self, request: &EppRequest) -> Result<String, Box<dyn Error>> {
        let epp_xml = request.to_epp_xml()?;

        println!("Request:\r\n{}", epp_xml);

        let response = self.connection.transact(&epp_xml).await?;
        println!("Response:\r\n{}", response);

        Ok(response)
    }

    pub async fn transact_xml(&mut self, xml: &str) -> Result<String, Box<dyn Error>> {
        self.connection.transact(&xml).await
    }

    pub fn greeting(&self) -> String {
        return String::from(&self.connection.greeting)
    }
}

async fn epp_connect(registry_creds: &EppClientConnection) -> Result<TlsStream<TcpStream>, error::Error> {
    let (host, port) = registry_creds.connection_details();

    println!("{}: {}", host, port);

    let addr = (host.as_str(), port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| stdio::ErrorKind::NotFound)?;

    let mut config = ClientConfig::new();

    config
        .root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr).await?;

    let domain = DNSNameRef::try_from_ascii_str(&host)
        .map_err(|_| stdio::Error::new(stdio::ErrorKind::InvalidInput, format!("Invalid domain: {}", host)))?;

    let stream = connector.connect(domain, stream).await?;

    Ok(stream)
}

pub async fn connect(registry: &'static str) -> Result<EppClient, Box<dyn Error>> {
    let registry_creds = match CONFIG.registry(registry) {
        Some(creds) => creds,
        None => return Err(format!("missing credentials for {}", registry).into())
    };

    let (tx, rx) = mpsc::channel();

    tokio::spawn(async move {
        let stream = epp_connect(&registry_creds).await.unwrap();
        let credentials = registry_creds.credentials();

        let connection = EppConnection::new(
            registry.to_string(),
            credentials,
            stream
        ).await.unwrap();

        let client = EppClient { connection: connection };

        tx.send(client).unwrap();
    });

    let client = rx.recv()?;

    Ok(client)
}
