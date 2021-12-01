//! Config
//!
//! This module contains the connection configuration for the EPP client.
//!
//! ## Example
//!
//! ```no_run
//! use std::collections::HashMap;
//!
//! use epp_client::config::{EppClientConfig, EppClientConnection};
//!
//! // Create a config
//! let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
//! registry.insert(
//!     "registry_name".to_owned(),
//!     EppClientConnection {
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
//! // Get configuration for the relevant registry section
//! let registry = config.registry("verisign").unwrap();
//!
//! // Get EPP host name and port no.
//! let remote = registry.connection_details();
//!
//! // Get username and password
//! let credentials = registry.credentials();
//!
//! // Get EPP service extensions
//! let service_extensions = registry.ext_uris().unwrap();
//! ```

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Paths to the client certificate and client key PEM files
#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientTlsFiles {
    pub cert_chain: String,
    pub key: String,
}

/// Config that stores settings for multiple registries
#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConfig {
    pub registry: HashMap<String, EppClientConnection>,
}

impl EppClientConfig {
    /// Returns the config for a particular registry
    pub fn registry(&self, registry: &str) -> Option<&EppClientConnection> {
        self.registry.get(registry)
    }
}

/// Connection details to connect to and authenticate with a registry
#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConnection {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub ext_uris: Option<Vec<String>>,
    pub tls_files: Option<EppClientTlsFiles>,
}

impl EppClientConnection {
    /// Returns the EPP host and port no as a tuple
    pub fn connection_details(&self) -> (String, u16) {
        (self.host.to_string(), self.port)
    }
    /// Returns the EPP username and password as a tuple
    pub fn credentials(&self) -> (String, String) {
        (self.username.to_string(), self.password.to_string())
    }
    /// Returns the service extension URIs to be set in the connection to the registry
    pub fn ext_uris(&self) -> Option<&Vec<String>> {
        self.ext_uris.as_ref()
    }
}
