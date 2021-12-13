//! Manages registry connections and reading/writing to them

use std::convert::TryInto;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{error::Error, io as stdio};
use std::{io, str, u32};

use rustls::{OwnedTrustAnchor, RootCertStore};
use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};
use tokio_rustls::{client::TlsStream, rustls::ClientConfig, TlsConnector};
use tracing::{debug, info};

use crate::common::{Certificate, PrivateKey};
use crate::error;

/// EPP Connection struct with some metadata for the connection
pub(crate) struct EppConnection {
    registry: String,
    stream: TlsStream<TcpStream>,
    pub greeting: String,
}

impl EppConnection {
    /// Create an EppConnection instance with the stream to the registry
    pub(crate) async fn connect(
        registry: String,
        addr: SocketAddr,
        hostname: &str,
        identity: Option<(Vec<Certificate>, PrivateKey)>,
    ) -> Result<EppConnection, Box<dyn Error>> {
        let mut stream = epp_connect(addr, hostname, identity).await?;

        let mut buf = vec![0u8; 4096];
        stream.read(&mut buf).await?;
        let greeting = str::from_utf8(&buf[4..])?.to_string();

        debug!("{}: greeting: {}", registry, greeting);

        Ok(EppConnection {
            registry,
            stream,
            greeting,
        })
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

        let wrote = self.stream.write(&buf).await?;
        debug!("{}: Wrote {} bytes", self.registry, wrote);
        Ok(())
    }

    /// Receives response from the socket and converts it into an EPP XML string
    async fn get_epp_response(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).await?;

        let buf_size: usize = u32::from_be_bytes(buf).try_into()?;

        let message_size = buf_size - 4;
        debug!("{}: Response buffer size: {}", self.registry, message_size);

        let mut buf = vec![0; message_size];
        let mut read_size: usize = 0;

        loop {
            let read = self.stream.read(&mut buf[read_size..]).await?;
            debug!("{}: Read: {} bytes", self.registry, read);

            read_size += read;
            debug!("{}: Total read: {} bytes", self.registry, read_size);

            if read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!("{}: unexpected eof", self.registry),
                )
                .into());
            } else if read_size >= message_size {
                break;
            }
        }

        Ok(String::from_utf8(buf)?)
    }

    /// Sends an EPP XML request to the registry and return the response
    /// receieved to the request
    pub(crate) async fn transact(&mut self, content: &str) -> Result<String, Box<dyn Error>> {
        debug!("{}: request: {}", self.registry, content);
        self.send_epp_request(content).await?;

        let response = self.get_epp_response().await?;
        debug!("{}: response: {}", self.registry, response);

        Ok(response)
    }

    /// Closes the socket and shuts the connection
    pub(crate) async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        info!("{}: Closing connection", self.registry);

        self.stream.shutdown().await?;
        Ok(())
    }
}

/// Establishes a TLS connection to a registry and returns a ConnectionStream instance containing the
/// socket stream to read/write to the connection
async fn epp_connect(
    addr: SocketAddr,
    hostname: &str,
    identity: Option<(Vec<Certificate>, PrivateKey)>,
) -> Result<TlsStream<TcpStream>, error::Error> {
    info!("Connecting to server: {:?}", addr,);

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

    let config = match identity {
        Some((certs, key)) => {
            let certs = certs
                .into_iter()
                .map(|cert| rustls::Certificate(cert.0))
                .collect();
            builder
                .with_single_cert(certs, rustls::PrivateKey(key.0))
                .map_err(|e| error::Error::Other(e.to_string()))?
        }
        None => builder.with_no_client_auth(),
    };

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr).await?;

    let domain = hostname.try_into().map_err(|_| {
        stdio::Error::new(
            stdio::ErrorKind::InvalidInput,
            format!("Invalid domain: {}", hostname),
        )
    })?;

    Ok(connector.connect(domain, stream).await?)
}
