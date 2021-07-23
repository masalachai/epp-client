use std::sync::Arc;
use std::{str, u32};
use bytes::BytesMut;
use std::convert::TryInto;
use futures::executor::block_on;
use std::{error::Error, net::ToSocketAddrs, io as stdio};
use tokio_rustls::{TlsConnector, rustls::ClientConfig, webpki::DNSNameRef, client::TlsStream};
use tokio::{net::TcpStream, io::AsyncWriteExt, io::AsyncReadExt, io::split, io::ReadHalf, io::WriteHalf};

use crate::config::{EppClientConnection};
use crate::error;

pub struct ConnectionStream {
    reader: ReadHalf<TlsStream<TcpStream>>,
    writer: WriteHalf<TlsStream<TcpStream>>,
}

pub struct EppConnection {
    registry: String,
    stream: ConnectionStream,
    pub greeting: String,
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

pub async fn epp_connect(registry_creds: &EppClientConnection) -> Result<ConnectionStream, error::Error> {
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
