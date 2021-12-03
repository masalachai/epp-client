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
//! let domain_check = DomainCheck::<NoExtension>::new(vec!["eppdev.com", "eppdev.net"]);
//! let response = client.transact(domain_check, "transaction-id").await.unwrap();
//! println!("{:?}", response);
//!
//! }
//! ```

use std::{error::Error, fmt::Debug};

use crate::common::EppObject;
use crate::config::EppClientConfig;
use crate::error;
use crate::hello::{Greeting, Hello};
use crate::registry::{epp_connect, EppConnection};
use crate::request::{EppExtension, EppRequest};
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

        let stream = epp_connect(registry_creds).await?;
        let connection = EppConnection::new(registry.to_string(), stream).await?;

        Ok(EppClient { connection })
    }

    /// Executes an EPP Hello call and returns the response as an `Greeting`
    pub async fn hello(&mut self) -> Result<Greeting, Box<dyn Error>> {
        let hello_xml = EppObject::<Hello>::build(Hello).serialize()?;

        let response = self.connection.transact(&hello_xml).await?;

        Ok(EppObject::<Greeting>::deserialize(&response)?.data)
    }

    pub async fn transact<T, E>(
        &mut self,
        request: T,
        id: &str,
    ) -> Result<Response<<T as EppRequest<E>>::Output, E::Response>, error::Error>
    where
        T: EppRequest<E> + Debug,
        E: EppExtension,
    {
        let epp_xml = request.serialize_request(id)?;

        let response = self.connection.transact(&epp_xml).await?;

        T::deserialize_response(&response)
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
        EppObject::<Greeting>::deserialize(&self.connection.greeting).map(|obj| obj.data)
    }
}
