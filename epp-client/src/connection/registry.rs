//! Manages registry connections and reading/writing to them

use bytes::BytesMut;
use rustls::{OwnedTrustAnchor, RootCertStore};
use std::convert::TryInto;
use std::sync::Arc;
use std::{error::Error, io as stdio, net::ToSocketAddrs};
use std::{str, u32};
use tokio::{
    io::split, io::AsyncReadExt, io::AsyncWriteExt, io::ReadHalf, io::WriteHalf, net::TcpStream,
};
use tokio_rustls::{client::TlsStream, rustls::ClientConfig, TlsConnector};

use crate::config::EppClientConnection;
use crate::error;

/// Socket stream for the connection to the registry
pub struct ConnectionStream {
    reader: ReadHalf<TlsStream<TcpStream>>,
    writer: WriteHalf<TlsStream<TcpStream>>,
}

/// EPP Connection struct with some metadata for the connection
pub struct EppConnection {
    registry: String,
    stream: ConnectionStream,
    pub greeting: String,
}

impl EppConnection {
    /// Create an EppConnection instance with the stream to the registry
    pub async fn new(
        registry: String,
        mut stream: ConnectionStream,
    ) -> Result<EppConnection, Box<dyn Error>> {
        let mut buf = vec![0u8; 4096];
        stream.reader.read(&mut buf).await?;
        let greeting = str::from_utf8(&buf[4..])?.to_string();

        debug!("{}: greeting: {}", registry, greeting);

        Ok(EppConnection {
            registry,
            stream,
            greeting,
        })
    }

    /// Writes to the socket
    async fn write(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        let wrote = self.stream.writer.write(buf).await?;

        debug!("{}: Wrote {} bytes", self.registry, wrote);

        Ok(())
    }

    /// Constructs an EPP XML request in the required form and sends it to the server
    async fn send_epp_request(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        let len = content.len();

        let buf_size = len + 4;
        let mut buf: Vec<u8> = vec![0u8; buf_size];

        let len = len + 4;
        let len_u32: [u8; 4] = u32::to_be_bytes(len.try_into()?);

        buf[..4].clone_from_slice(&len_u32);
        buf[4..].clone_from_slice(content.as_bytes());

        self.write(&buf).await
    }

    /// Reads response from the socket
    async fn read_epp_response(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = [0u8; 4];
        self.stream.reader.read_exact(&mut buf).await?;

        let buf_size: usize = u32::from_be_bytes(buf).try_into()?;

        let message_size = buf_size - 4;
        debug!("{}: Response buffer size: {}", self.registry, message_size);

        let mut buf = BytesMut::with_capacity(4096);
        let mut read_buf = vec![0u8; 4096];

        let mut read_size: usize = 0;

        loop {
            let read = self.stream.reader.read(&mut read_buf).await?;
            debug!("{}: Read: {} bytes", self.registry, read);
            buf.extend_from_slice(&read_buf[0..read]);

            read_size += read;
            debug!("{}: Total read: {} bytes", self.registry, read_size);

            if read == 0 {
                panic!("{}: Unexpected eof", self.registry)
            } else if read_size >= message_size {
                break;
            }
        }

        let data = buf.to_vec();

        Ok(data)
    }

    /// Receives response from the socket and converts it into an EPP XML string
    async fn get_epp_response(&mut self) -> Result<String, Box<dyn Error>> {
        let contents = self.read_epp_response().await?;

        let response = str::from_utf8(&contents)?.to_string();

        Ok(response)
    }

    /// Sends an EPP XML request to the registry and return the response
    /// receieved to the request
    pub async fn transact(&mut self, content: &str) -> Result<String, Box<dyn Error>> {
        debug!("{}: request: {}", self.registry, content);
        self.send_epp_request(content).await?;

        let response = self.get_epp_response().await?;
        debug!("{}: response: {}", self.registry, response);

        Ok(response)
    }

    /// Closes the socket and shuts the connection
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        info!("{}: Closing connection", self.registry);

        self.stream.writer.shutdown().await?;
        Ok(())
    }
}

/// Establishes a TLS connection to a registry and returns a ConnectionStream instance containing the
/// socket stream to read/write to the connection
pub async fn epp_connect(
    registry_creds: &EppClientConnection,
) -> Result<ConnectionStream, error::Error> {
    let (host, port) = registry_creds.connection_details();

    info!("Connecting: EPP Server: {} Port: {}", host, port);

    let addr = (host.as_str(), port)
        .to_socket_addrs()?
        .next()
        .ok_or(stdio::ErrorKind::NotFound)?;

    let mut roots = RootCertStore::empty();
    roots.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let builder = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots);

    let config = match registry_creds.tls_files() {
        Some((cert_chain, key)) => match builder.with_single_cert(cert_chain, key) {
            Ok(config) => config,
            Err(e) => return Err(format!("Failed to set client TLS credentials: {}", e).into()),
        },
        None => builder.with_no_client_auth(),
    };

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr).await?;

    let domain = host.as_str().try_into().map_err(|_| {
        stdio::Error::new(
            stdio::ErrorKind::InvalidInput,
            format!("Invalid domain: {}", host),
        )
    })?;

    let stream = connector.connect(domain, stream).await?;

    let (reader, writer) = split(stream);

    Ok(ConnectionStream { reader, writer })
}
