//! Manages sending/receiving EppObject request and responses to the registry connection
//!
//! ## Example
//!
//! ```no_run
//! use std::collections::HashMap;
//!
//! use epp_client::config::{EppClientConfig, RegistryConfig};
//! use epp_client::EppClient;
//! use epp_client::domain::check::DomainCheck;
//! use epp_client::common::NoExtension;
//!
//! #[tokio::main]
//! async fn main() {
//!
//! // Create a config
//! let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
//! registry.insert(
//!     "registry_name".to_owned(),
//!     RegistryConfig {
//!         host: "example.com".to_owned(),
//!         port: 700,
//!         tls_files: None,
//!     },
//! );
//! let config = EppClientConfig { registry };
//!
//! // Create an instance of EppClient, passing the config and the registry you want to connect to
//! let mut client = match EppClient::new(&config, "registry_name").await {
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

use std::error::Error;

use crate::common::NoExtension;
use crate::config::EppClientConfig;
use crate::connection::EppConnection;
use crate::error;
use crate::hello::{Greeting, GreetingDocument, HelloDocument};
use crate::request::{Command, Extension, Transaction};
use crate::response::Response;
use crate::xml::EppXml;

/// Instances of the EppClient type are used to transact with the registry.
/// Once initialized, the EppClient instance can serialize EPP requests to XML and send them
/// to the registry and deserialize the XML responses from the registry to local types
pub struct EppClient {
    connection: EppConnection,
}

impl EppClient {
    /// Creates a new EppClient object and does an EPP Login to a given registry to become ready
    /// for subsequent transactions on this client instance
    pub async fn new(
        config: &EppClientConfig,
        registry: &str,
    ) -> Result<EppClient, Box<dyn Error>> {
        let registry_creds = match config.registry(registry) {
            Some(creds) => creds,
            None => return Err(format!("missing credentials for {}", registry).into()),
        };

        Ok(EppClient {
            connection: EppConnection::connect(registry.to_string(), registry_creds).await?,
        })
    }

    /// Executes an EPP Hello call and returns the response as an `Greeting`
    pub async fn hello(&mut self) -> Result<Greeting, Box<dyn Error>> {
        let hello_xml = HelloDocument::default().serialize()?;

        let response = self.connection.transact(&hello_xml).await?;

        Ok(GreetingDocument::deserialize(&response)?.data)
    }

    pub async fn transact<'a, C: 'a, E: 'a>(
        &mut self,
        data: impl Into<RequestData<'a, C, E>> + 'a,
        id: &str,
    ) -> Result<Response<C::Response, E::Response>, error::Error>
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
    pub async fn transact_xml(&mut self, xml: &str) -> Result<String, Box<dyn Error>> {
        self.connection.transact(xml).await
    }

    /// Returns the greeting received on establishment of the connection in raw xml form
    pub fn xml_greeting(&self) -> String {
        String::from(&self.connection.greeting)
    }

    /// Returns the greeting received on establishment of the connection as an `Greeting`
    pub fn greeting(&self) -> Result<Greeting, error::Error> {
        GreetingDocument::deserialize(&self.connection.greeting).map(|obj| obj.data)
    }

    pub async fn shutdown(mut self) -> Result<(), Box<dyn Error>> {
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
