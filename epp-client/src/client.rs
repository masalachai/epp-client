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
//! use epp_client::generate_client_tr_id;
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
//!         username: "username".to_owned(),
//!         password: "password".to_owned(),
//!         ext_uris: None,
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
//! let response = client.transact(domain_check, generate_client_tr_id(&client).as_str()).await.unwrap();
//! println!("{:?}", response);
//!
//! }
//! ```

use std::time::SystemTime;
use std::{error::Error, fmt::Debug};

use crate::common::{EppObject, NoExtension};
use crate::config::EppClientConfig;
use crate::error;
use crate::hello::{Greeting, Hello};
use crate::login::Login;
use crate::logout::Logout;
use crate::registry::{epp_connect, EppConnection};
use crate::request::{generate_client_tr_id, EppExtension, EppRequest};
use crate::response::{Response, ResponseStatus};
use crate::xml::EppXml;

/// Instances of the EppClient type are used to transact with the registry.
/// Once initialized, the EppClient instance can serialize EPP requests to XML and send them
/// to the registry and deserialize the XML responses from the registry to local types
pub struct EppClient {
    credentials: (String, String),
    ext_uris: Option<Vec<String>>,
    connection: EppConnection,
}

/// A function to generate a simple client TRID. Should only be used for testing, library users
/// should generate a client TRID according to their own requirements
pub fn default_client_tr_id_fn(client: &EppClient) -> String {
    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => time,
        Err(e) => panic!("Error in client TRID gen function: {}", e),
    };
    format!("{}:{}", &client.username(), timestamp.as_secs())
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
        let credentials = registry_creds.credentials();
        let ext_uris = registry_creds.ext_uris();

        let ext_uris =
            ext_uris.map(|uris| uris.iter().map(|u| u.to_string()).collect::<Vec<String>>());

        let connection = EppConnection::new(registry.to_string(), stream).await?;
        EppClient::build(connection, credentials, ext_uris).await
    }

    /// Makes a login request to the registry and initializes an EppClient instance with it
    async fn build(
        connection: EppConnection,
        credentials: (String, String),
        ext_uris: Option<Vec<String>>,
    ) -> Result<EppClient, Box<dyn Error>> {
        let mut client = EppClient {
            connection,
            credentials,
            ext_uris,
        };

        let client_tr_id = generate_client_tr_id(&client.credentials.0)?;
        let login_request = Login::<NoExtension>::new(
            &client.credentials.0,
            &client.credentials.1,
            &client.ext_uris,
        );

        client
            .transact(login_request, client_tr_id.as_str())
            .await?;

        Ok(client)
    }

    /// Executes an EPP Hello call and returns the response as an `EppGreeting`
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

    /// Fetches the username used in the registry connection
    pub fn username(&self) -> String {
        self.credentials.0.to_string()
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

    /// Returns the greeting received on establishment of the connection as an `EppGreeting`
    pub fn greeting(&self) -> Result<Greeting, error::Error> {
        EppObject::<Greeting>::deserialize(&self.connection.greeting).map(|obj| obj.data)
    }

    /// Sends the EPP Logout command to log out of the EPP session
    pub async fn logout(&mut self) -> Result<Response<ResponseStatus, NoExtension>, error::Error> {
        let client_tr_id = generate_client_tr_id(&self.credentials.0).unwrap();
        let epp_logout = Logout::<NoExtension>::new();

        let response = self.transact(epp_logout, client_tr_id.as_str()).await?;

        self.connection.shutdown().await?;

        Ok(response)
    }
}
