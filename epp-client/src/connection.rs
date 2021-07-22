use std::sync::Arc;
use std::sync::mpsc;
use std::{str, u32};
use bytes::BytesMut;
use std::convert::TryInto;
use futures::executor::block_on;
use std::{error::Error, fmt::Debug, net::ToSocketAddrs, io as stdio};
use tokio_rustls::{TlsConnector, rustls::ClientConfig, webpki::DNSNameRef, client::TlsStream};
use tokio::{net::TcpStream, io::AsyncWriteExt, io::AsyncReadExt, io::split, io::ReadHalf, io::WriteHalf};

use crate::config::{CONFIG, EppClientConnection};
use crate::error;
use crate::epp::request::{generate_client_tr_id, EppLogin, EppLogout};
use crate::epp::response::EppCommandResponse;
use crate::epp::xml::EppXml;

pub struct ConnectionStream {
    reader: ReadHalf<TlsStream<TcpStream>>,
    writer: WriteHalf<TlsStream<TcpStream>>,
}

pub struct EppConnection {
    registry: String,
    stream: ConnectionStream,
    pub greeting: String,
}

pub struct EppClient {
    credentials: (String, String),
    connection: EppConnection,
}

impl EppConnection {
    pub async fn new(
        registry: String,
        mut stream: ConnectionStream) -> Result<EppConnection, Box<dyn Error>> {
        let mut buf = vec![0u8; 4096];
        stream.reader.read(&mut buf).await?;
        let greeting = str::from_utf8(&buf)?.to_string();

        Ok(EppConnection {
            registry: registry,
            stream: stream,
            greeting: greeting
        })
    }

    async fn write(&mut self, buf: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let wrote = self.stream.writer.write(buf).await?;

        println!("Wrote {} bytes", wrote);

        Ok(())
    }

    pub async fn send_epp_request(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        let len = content.len();

        let buf_size = len + 4;
        let mut buf: Vec<u8> = vec![0u8; buf_size];

        let len = len + 4;
        let len_u32: [u8; 4] = u32::to_be_bytes(len.try_into()?);

        buf[..4].clone_from_slice(&len_u32);
        buf[4..].clone_from_slice(&content.as_bytes());

        self.write(&buf).await
    }

    async fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = vec![0u8; 4096];
        self.stream.reader.read(&mut buf).await?;
        Ok(buf)
    }

    async fn read_epp_response(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = [0u8; 4];
        self.stream.reader.read_exact(&mut buf).await?;

        let buf_size :usize = u32::from_be_bytes(buf).try_into()?;

        let message_size = buf_size - 4;
        println!("Message buffer size: {}", message_size);

        let mut buf = BytesMut::with_capacity(4096);
        let mut read_buf = vec![0u8; 4096];

        let mut read_size :usize = 0;

        loop {
            let read = self.stream.reader.read(&mut read_buf).await?;
            println!("Read: {} bytes", read);
            buf.extend_from_slice(&read_buf[0..read]);

            read_size = read_size + read;
            println!("Total read: {} bytes", read_size);

            if read == 0 {
                panic!("Unexpected eof")
            } else if read_size >= message_size {
                break;
            }
        }

        let data = buf.to_vec();

        Ok(data)
    }

    pub async fn get_epp_response(&mut self) -> Result<String, Box<dyn Error>> {
        let contents = self.read_epp_response().await?;

        let response = str::from_utf8(&contents)?.to_string();

        Ok(response)
    }

    pub async fn transact(&mut self, content: &str) -> Result<String, Box<dyn Error>> {
        self.send_epp_request(&content).await?;

        self.get_epp_response().await
    }

    async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Closing ...");

        self.stream.writer.shutdown().await?;
        Ok(())
    }
}

impl Drop for EppConnection {
    fn drop(&mut self) {
        block_on(self.close());
    }
}

impl EppClient {
    pub async fn new(connection: EppConnection, credentials: (String, String)) -> Result<EppClient, Box<dyn Error>> {
        let mut client = EppClient {
            connection: connection,
            credentials: credentials
        };

        let client_tr_id = generate_client_tr_id(&client.credentials.0)?;
        let login_request = EppLogin::new(&client.credentials.0, &client.credentials.1, client_tr_id.as_str());

        client.transact::<EppLogin, EppCommandResponse>(&login_request).await?;

        Ok(client)
    }

    pub async fn transact<T: EppXml + Debug, E: EppXml + Debug>(&mut self, request: &T) -> Result<E::Output, Box<dyn Error>> {
        let epp_xml = request.serialize()?;

        println!("Request:\r\n{}", epp_xml);

        let response = self.connection.transact(&epp_xml).await?;

        println!("Response:\r\n{}", response);

        // let result_object = EppCommandResponse::deserialize(&response);

        let response_obj = E::deserialize(&response)?;

        println!("Response:\r\n{:?}", response_obj);

        Ok(response_obj)
    }

    pub async fn transact_xml(&mut self, xml: &str) -> Result<String, Box<dyn Error>> {
        self.connection.transact(&xml).await
    }

    pub fn greeting(&self) -> String {
        return String::from(&self.connection.greeting)
    }

    pub async fn logout(&mut self) {
        let client_tr_id = generate_client_tr_id(&self.credentials.0).unwrap();
        let epp_logout = EppLogout::new(client_tr_id.as_str());

        self.transact::<EppLogout, EppCommandResponse>(&epp_logout).await;
    }
}

impl Drop for EppClient {
    fn drop(&mut self) {
        block_on(self.logout());
    }
}

async fn epp_connect(registry_creds: &EppClientConnection) -> Result<ConnectionStream, error::Error> {
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

    let (reader, writer) = split(stream);

    Ok(ConnectionStream {
        reader: reader,
        writer: writer,
    })
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
            stream
        ).await.unwrap();

        let client = EppClient::new(connection, credentials).await.unwrap();

        tx.send(client).unwrap();
    });

    let client = rx.recv()?;

    Ok(client)
}
