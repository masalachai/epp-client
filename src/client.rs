use std::convert::TryInto;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tokio::net::TcpStream;
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::client::TlsStream;
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName};
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::TlsConnector;
use tracing::info;

use crate::common::{Certificate, NoExtension, PrivateKey};
pub use crate::connection::Connector;
use crate::connection::{self, EppConnection};
use crate::error::Error;
use crate::hello::{Greeting, GreetingDocument, HelloDocument};
use crate::request::{Command, Extension, Transaction};
use crate::response::Response;
use crate::xml::EppXml;

/// An `EppClient` provides an interface to sending EPP requests to a registry
///
/// Once initialized, the EppClient instance can serialize EPP requests to XML and send them
/// to the registry and deserialize the XML responses from the registry to local types.
///
/// # Examples
///
/// ```no_run
/// # use std::collections::HashMap;
/// # use std::net::ToSocketAddrs;
/// # use std::time::Duration;
/// #
/// use epp_client::EppClient;
/// use epp_client::domain::DomainCheck;
/// use epp_client::common::NoExtension;
///
/// # #[tokio::main]
/// # async fn main() {
/// // Create an instance of EppClient
/// let host = "example.com";
/// let addr = (host, 7000).to_socket_addrs().unwrap().next().unwrap();
/// let timeout = Duration::from_secs(5);
/// let mut client = match EppClient::connect("registry_name".to_string(), addr, host, None, timeout).await {
///     Ok(client) => client,
///     Err(e) => panic!("Failed to create EppClient: {}",  e)
/// };
///
/// // Make a EPP Hello call to the registry
/// let greeting = client.hello().await.unwrap();
/// println!("{:?}", greeting);
///
/// // Execute an EPP Command against the registry with distinct request and response objects
/// let domain_check = DomainCheck { domains: &["eppdev.com", "eppdev.net"] };
/// let response = client.transact(&domain_check, "transaction-id").await.unwrap();
/// response.res_data.unwrap().list
///     .iter()
///     .for_each(|chk| println!("Domain: {}, Available: {}", chk.id, chk.available));
/// # }
/// ```
///
/// The output would look like this:
///
/// ```text
/// Domain: eppdev.com, Available: 1
/// Domain: eppdev.net, Available: 1
/// ```
pub struct EppClient<C: Connector> {
    connection: EppConnection<C>,
}

#[cfg(feature = "tokio-rustls")]
impl EppClient<RustlsConnector> {
    /// Connect to the specified `addr` and `hostname` over TLS
    ///
    /// The `registry` is used as a name in internal logging; `addr` provides the address to
    /// connect to, `hostname` is sent as the TLS server name indication and `identity` provides
    /// optional TLS client authentication (using) rustls as the TLS implementation.
    /// The `timeout` limits the time spent on any underlying network operations.
    ///
    /// Alternatively, use `EppClient::new()` with any established `AsyncRead + AsyncWrite + Unpin`
    /// implementation.
    pub async fn connect(
        registry: String,
        addr: SocketAddr,
        hostname: &str,
        identity: Option<(Vec<Certificate>, PrivateKey)>,
        timeout: Duration,
    ) -> Result<Self, Error> {
        info!("Connecting to server: {:?}", addr);
        let connector = RustlsConnector::new(addr, hostname, identity)?;
        Self::new(connector, registry, timeout).await
    }
}

impl<C: Connector> EppClient<C> {
    /// Create an `EppClient` from an already established connection
    pub async fn new(connector: C, registry: String, timeout: Duration) -> Result<Self, Error> {
        Ok(Self {
            connection: EppConnection::new(connector, registry, timeout).await?,
        })
    }

    /// Executes an EPP Hello call and returns the response as an `Greeting`
    pub async fn hello(&mut self) -> Result<Greeting, Error> {
        let hello_xml = HelloDocument::default().serialize()?;

        let response = self.connection.transact(&hello_xml).await?;

        Ok(GreetingDocument::deserialize(&response)?.data)
    }

    pub async fn transact<'c, 'e, Cmd, Ext>(
        &mut self,
        data: impl Into<RequestData<'c, 'e, Cmd, Ext>>,
        id: &str,
    ) -> Result<Response<Cmd::Response, Ext::Response>, Error>
    where
        Cmd: Transaction<Ext> + Command + 'c,
        Ext: Extension + 'e,
    {
        let data = data.into();
        let epp_xml =
            <Cmd as Transaction<Ext>>::serialize_request(data.command, data.extension, id)?;

        let response = self.connection.transact(&epp_xml).await?;

        Cmd::deserialize_response(&response)
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

    pub async fn reconnect(&mut self) -> Result<(), Error> {
        self.connection.reconnect().await
    }

    pub async fn shutdown(mut self) -> Result<(), Error> {
        self.connection.shutdown().await
    }
}

pub struct RequestData<'c, 'e, C, E> {
    command: &'c C,
    extension: Option<&'e E>,
}

impl<'c, C: Command> From<&'c C> for RequestData<'c, 'static, C, NoExtension> {
    fn from(command: &'c C) -> Self {
        Self {
            command,
            extension: None,
        }
    }
}

impl<'c, 'e, C: Command, E: Extension> From<(&'c C, &'e E)> for RequestData<'c, 'e, C, E> {
    fn from((command, extension): (&'c C, &'e E)) -> Self {
        Self {
            command,
            extension: Some(extension),
        }
    }
}

#[cfg(feature = "tokio-rustls")]
pub struct RustlsConnector {
    inner: TlsConnector,
    domain: ServerName,
    addr: SocketAddr,
}

impl RustlsConnector {
    pub fn new(
        addr: SocketAddr,
        hostname: &str,
        identity: Option<(Vec<Certificate>, PrivateKey)>,
    ) -> Result<Self, Error> {
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
                    .map(|cert| tokio_rustls::rustls::Certificate(cert.0))
                    .collect();
                builder
                    .with_single_cert(certs, tokio_rustls::rustls::PrivateKey(key.0))
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

        Ok(Self {
            inner: TlsConnector::from(Arc::new(config)),
            domain,
            addr,
        })
    }
}

#[cfg(feature = "tokio-rustls")]
#[async_trait]
impl Connector for RustlsConnector {
    type Connection = TlsStream<TcpStream>;

    async fn connect(&self, timeout: Duration) -> Result<Self::Connection, Error> {
        let stream = TcpStream::connect(&self.addr).await?;
        let future = self.inner.connect(self.domain.clone(), stream);
        connection::timeout(timeout, future).await
    }
}
