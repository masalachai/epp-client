//! Manages sending/receiving EppObject request and responses to the registry connection
//!
//! ## Example
//!
//! ```no_run
//! use std::collections::HashMap;
//! use std::net::ToSocketAddrs;
//!
//! use epp_client::EppClient;
//! use epp_client::domain::check::DomainCheck;
//! use epp_client::common::NoExtension;
//!
//! #[tokio::main]
//! async fn main() {
//!
//! // Create an instance of EppClient
//! let host = "example.com";
//! let addr = (host, 7000).to_socket_addrs().unwrap().next().unwrap();
//! let mut client = match EppClient::connect("registry_name".to_string(), addr, host, None).await {
//!     Ok(client) => client,
//!     Err(e) => panic!("Failed to create EppClient: {}",  e)
//! };
//!
//! // Make a EPP Hello call to the registry
//! let greeting = client.hello().await.unwrap();
//! println!("{:?}", greeting);
//!
//! // Execute an EPP Command against the registry with distinct request and response objects
//! let domain_check = DomainCheck::new(vec!["eppdev.com", "eppdev.net"]);
//! let response = client.transact(&domain_check, "transaction-id").await.unwrap();
//! println!("{:?}", response);
//!
//! }
//! ```

use std::convert::TryInto;
use std::net::SocketAddr;
use std::sync::Arc;
use std::io;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
use tokio_rustls::TlsConnector;
use tracing::info;

use crate::common::{Certificate, NoExtension, PrivateKey};
use crate::connection::EppConnection;
use crate::error::Error;
use crate::hello::{Greeting, GreetingDocument, HelloDocument};
use crate::request::{Command, Extension, Transaction};
use crate::response::Response;
use crate::xml::EppXml;

/// Instances of the EppClient type are used to transact with the registry.
/// Once initialized, the EppClient instance can serialize EPP requests to XML and send them
/// to the registry and deserialize the XML responses from the registry to local types
pub struct EppClient<IO> {
    connection: EppConnection<IO>,
}

impl EppClient<TlsStream<TcpStream>> {
    /// Connect to the specified `addr` and `hostname` over TLS
    ///
    /// The `registry` is used as a name in internal logging; `addr` provides the address to
    /// connect to, `hostname` is sent as the TLS server name indication and `identity` provides
    /// optional TLS client authentication. Uses rustls as the TLS implementation.
    ///
    /// Alternatively, use `EppClient::new()` with any established `AsyncRead + AsyncWrite + Unpin`
    /// implementation.
    pub async fn connect(
        registry: String,
        addr: SocketAddr,
        hostname: &str,
        identity: Option<(Vec<Certificate>, PrivateKey)>,
    ) -> Result<Self, Error> {
        info!("Connecting to server: {:?}", addr);

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
                    .map_err(|e| Error::Other(e.into()))?
            }
            None => builder.with_no_client_auth(),
        };

        let domain = hostname.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid domain: {}", hostname),
            )
        })?;

        let connector = TlsConnector::from(Arc::new(config));
        let tcp = TcpStream::connect(&addr).await?;
        let stream = connector.connect(domain, tcp).await?;
        Self::new(registry, stream).await
    }
}

impl<IO: AsyncRead + AsyncWrite + Unpin> EppClient<IO> {
    /// Create an `EppClient` from an already established connection
    pub async fn new(registry: String, stream: IO) -> Result<Self, Error> {
        Ok(Self {
            connection: EppConnection::new(registry, stream).await?,
        })
    }

    /// Executes an EPP Hello call and returns the response as an `Greeting`
    pub async fn hello(&mut self) -> Result<Greeting, Error> {
        let hello_xml = HelloDocument::default().serialize()?;

        let response = self.connection.transact(&hello_xml).await?;

        Ok(GreetingDocument::deserialize(&response)?.data)
    }

    pub async fn transact<'a, C: 'a, E: 'a>(
        &mut self,
        data: impl Into<RequestData<'a, C, E>> + 'a,
        id: &str,
    ) -> Result<Response<C::Response, E::Response>, Error>
    where
        C: Transaction<E> + Command,
        E: Extension,
    {
        let data = data.into();
        let epp_xml = <C as Transaction<E>>::serialize_request(data.command, data.extension, id)?;

        let response = self.connection.transact(&epp_xml).await?;

        C::deserialize_response(&response)
    }

    /// Accepts raw EPP XML and returns the raw EPP XML response to it.
    /// Not recommended for direct use but sometimes can be useful for debugging
    pub async fn transact_xml(&mut self, xml: &str) -> Result<String, Error> {
        self.connection.transact(xml).await
    }

    /// Returns the greeting received on establishment of the connection in raw xml form
    pub fn xml_greeting(&self) -> String {
        String::from(&self.connection.greeting)
    }

    /// Returns the greeting received on establishment of the connection as an `Greeting`
    pub fn greeting(&self) -> Result<Greeting, Error> {
        GreetingDocument::deserialize(&self.connection.greeting).map(|obj| obj.data)
    }

    pub async fn shutdown(mut self) -> Result<(), Error> {
        self.connection.shutdown().await
    }
}

pub struct RequestData<'a, C, E> {
    command: &'a C,
    extension: Option<&'a E>,
}

impl<'a, C: Command> From<&'a C> for RequestData<'a, C, NoExtension> {
    fn from(command: &'a C) -> Self {
        Self {
            command,
            extension: None,
        }
    }
}

impl<'a, C: Command, E: Extension> From<(&'a C, &'a E)> for RequestData<'a, C, E> {
    fn from((command, extension): (&'a C, &'a E)) -> Self {
        Self {
            command,
            extension: Some(extension),
        }
    }
}
