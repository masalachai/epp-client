//! Config
//!
//! This module contains the connection configuration for the EPP client.
//!
//! ## Example
//!
//! ```no_run
//! use std::collections::HashMap;
//!
//! use epp_client::config::{EppClientConfig, RegistryConfig};
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
//! // Get configuration for the relevant registry section
//! let registry = config.registry("verisign").unwrap();
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
    pub registry: HashMap<String, RegistryConfig>,
}

impl EppClientConfig {
    /// Returns the config for a particular registry
    pub fn registry(&self, registry: &str) -> Option<&RegistryConfig> {
        self.registry.get(registry)
    }
}

/// Connection details to connect to and authenticate with a registry
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryConfig {
    pub host: String,
    pub port: u16,
    pub tls_files: Option<EppClientTlsFiles>,
}
