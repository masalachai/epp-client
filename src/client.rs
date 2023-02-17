use std::io;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
#[cfg(feature = "tokio-rustls")]
use tokio::net::lookup_host;
use tokio::net::TcpStream;
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::client::TlsStream;
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName};
#[cfg(feature = "tokio-rustls")]
use tokio_rustls::TlsConnector;
use tracing::{debug, error, info};

use crate::common::{Certificate, NoExtension, PrivateKey};
pub use crate::connection::Connector;
use crate::connection::{self, EppConnection};
use crate::error::Error;
use crate::hello::{Greeting, GreetingDocument, HelloDocument};
use crate::request::{Command, CommandDocument, Extension, Transaction};
use crate::response::{Response, ResponseDocument, ResponseStatus};
use crate::xml;

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
/// let timeout = Duration::from_secs(5);
/// let mut client = match EppClient::connect("registry_name".to_string(), ("example.com".to_owned(), 7000), None, timeout).await {
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
    /// The `registry` is used as a name in internal logging; `host` provides the host name
    /// and port to connect to), `hostname` is sent as the TLS server name indication and
    /// `identity` provides optional TLS client authentication (using) rustls as the TLS
    /// implementation. The `timeout` limits the time spent on any underlying network operations.
    ///
    /// Alternatively, use `EppClient::new()` with any established `AsyncRead + AsyncWrite + Unpin`
    /// implementation.
    pub async fn connect(
        registry: String,
        server: (String, u16),
        identity: Option<(Vec<Certificate>, PrivateKey)>,
        timeout: Duration,
    ) -> Result<Self, Error> {
        let connector = RustlsConnector::new(server, identity).await?;
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

    /// Executes an EPP Hello call and returns the response as a `Greeting`
    pub async fn hello(&mut self) -> Result<Greeting, Error> {
        let xml = xml::serialize(&HelloDocument::default())?;

        debug!("{}: hello: {}", self.connection.registry, &xml);
        let response = self.connection.transact(&xml)?.await?;
        debug!("{}: greeting: {}", self.connection.registry, &response);

        Ok(xml::deserialize::<GreetingDocument>(&response)?.data)
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
        let document = CommandDocument::new(data.command, data.extension, id);
        let xml = xml::serialize(&document)?;

        debug!("{}: request: {}", self.connection.registry, &xml);
        let response = self.connection.transact(&xml)?.await?;
        debug!("{}: response: {}", self.connection.registry, &response);

        let rsp =
            match xml::deserialize::<ResponseDocument<Cmd::Response, Ext::Response>>(&response) {
                Ok(rsp) => rsp,
                Err(e) => {
                    error!(%response, "failed to deserialize response for transaction: {e}");
                    return Err(e);
                }
            };

        if rsp.data.result.code.is_success() {
            return Ok(rsp.data);
        }

        let err = crate::error::Error::Command(Box::new(ResponseStatus {
            result: rsp.data.result,
            tr_ids: rsp.data.tr_ids,
        }));

        error!(%response, "Failed to deserialize response for transaction: {}", err);
        Err(err)
    }

    /// Accepts raw EPP XML and returns the raw EPP XML response to it.
    /// Not recommended for direct use but sometimes can be useful for debugging
    pub async fn transact_xml(&mut self, xml: &str) -> Result<String, Error> {
        self.connection.transact(xml)?.await
    }

    /// Returns the greeting received on establishment of the connection in raw xml form
    pub fn xml_greeting(&self) -> String {
        String::from(&self.connection.greeting)
    }

    /// Returns the greeting received on establishment of the connection as an `Greeting`
    pub fn greeting(&self) -> Result<Greeting, Error> {
        xml::deserialize::<GreetingDocument>(&self.connection.greeting).map(|obj| obj.data)
    }

    pub async fn reconnect(&mut self) -> Result<(), Error> {
        self.connection.reconnect().await
    }

    pub async fn shutdown(mut self) -> Result<(), Error> {
        self.connection.shutdown().await
    }
}

#[derive(Debug)]
pub struct RequestData<'c, 'e, C, E> {
    pub(crate) command: &'c C,
    pub(crate) extension: Option<&'e E>,
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

// Manual impl because this does not depend on whether `C` and `E` are `Clone`
impl<'c, 'e, C, E> Clone for RequestData<'c, 'e, C, E> {
    fn clone(&self) -> Self {
        Self {
            command: self.command,
            extension: self.extension,
        }
    }
}

// Manual impl because this does not depend on whether `C` and `E` are `Copy`
impl<'c, 'e, C, E> Copy for RequestData<'c, 'e, C, E> {}

#[cfg(feature = "tokio-rustls")]
pub struct RustlsConnector {
    inner: TlsConnector,
    domain: ServerName,
    server: (String, u16),
}

impl RustlsConnector {
    pub async fn new(
        server: (String, u16),
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

        let domain = server.0.as_str().try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid domain: {}", server.0),
            )
        })?;

        Ok(Self {
            inner: TlsConnector::from(Arc::new(config)),
            domain,
            server,
        })
    }
}

#[cfg(feature = "tokio-rustls")]
#[async_trait]
impl Connector for RustlsConnector {
    type Connection = TlsStream<TcpStream>;

    async fn connect(&self, timeout: Duration) -> Result<Self::Connection, Error> {
        info!("Connecting to server: {}:{}", self.server.0, self.server.1);
        let addr = match lookup_host(&self.server).await?.next() {
            Some(addr) => addr,
            None => {
                return Err(Error::Io(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid host: {}", &self.server.0),
                )))
            }
        };

        let stream = TcpStream::connect(addr).await?;
        let future = self.inner.connect(self.domain.clone(), stream);
        connection::timeout(timeout, future).await
    }
}
