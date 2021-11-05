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
//!
//! // Get client certificate and private key
//! let tls = registry.tls_files().unwrap();
//! ```

use rustls::{Certificate, PrivateKey};
use rustls_pemfile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Seek, SeekFrom};
use std::{fs, io};

/// Paths to the client certificate and client key PEM files
#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientTlsFiles {
    cert_chain: String,
    key: String,
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

/// Config that stores settings for multiple registries
#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConfig {
    pub registry: HashMap<String, EppClientConnection>,
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
    /// Returns the parsed client certificate and private key for client TLS auth
    pub fn tls_files(&self) -> Option<(Vec<Certificate>, PrivateKey)> {
        let certificates = self.client_certificate();
        let key = self.key();

        if certificates == None || key == None {
            None
        } else {
            Some((certificates.unwrap(), key.unwrap()))
        }
    }
    /// Parses the client certificate chain
    fn client_certificate(&self) -> Option<Vec<Certificate>> {
        self.tls_files.as_ref().map(|tls| {
            rustls_pemfile::certs(&mut io::BufReader::new(
                fs::File::open(tls.cert_chain.to_string()).unwrap(),
            ))
            .unwrap()
            .iter()
            .map(|v| Certificate(v.clone()))
            .collect()
        })
    }
    /// Parses the client private key
    fn key(&self) -> Option<PrivateKey> {
        self.tls_files.as_ref().map(|tls| {
            let mut r = io::BufReader::new(fs::File::open(tls.key.to_string()).unwrap());

            let rsa_keys = rustls_pemfile::rsa_private_keys(&mut r).unwrap();
            if rsa_keys.len() > 1 {
                warn!("Multiple RSA keys found in PEM file {}", tls.key);
            } else if !rsa_keys.is_empty() {
                return rustls::PrivateKey(rsa_keys[0].clone());
            }

            r.seek(SeekFrom::Start(0)).unwrap();

            let pkcs8_keys = rustls_pemfile::pkcs8_private_keys(&mut r).unwrap();
            if pkcs8_keys.len() > 1 {
                warn!("Multiple PKCS8 keys found in PEM file {}", tls.key);
            } else if !pkcs8_keys.is_empty() {
                return rustls::PrivateKey(pkcs8_keys[0].clone());
            }

            panic!("No private key found in PEM file");
        })
    }
}

impl EppClientConfig {
    /// Returns the config for a particular registry
    pub fn registry(&self, registry: &str) -> Option<&EppClientConnection> {
        self.registry.get(registry)
    }
}
