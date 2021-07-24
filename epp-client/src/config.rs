//! Config load module
//!
//! Loads the configuration and credentials for each registry connection from
//! the `$XDG_CONFIG_HOME/epp-client/epp-client.toml` file
//!
//! ## Usage
//!
//! The config is automatically loaded when the module is initialized
//! and is available through the `epp_client::config::CONFIG` variable
//!
//! ## Sample config
//!
//! ```toml
//! [registry.verisign]
//! host = 'epp.verisign-grs.com'
//! port = 700
//! username = 'username'
//! password = 'password'
//! # service extensions
//! ext_uris = []
//!
//! [registry.hexonet.tls_files]
//! # the full client certificate chain in PEM format
//! cert_chain = '/path/to/certificate/chain/pemfile'
//! # the RSA private key for your certificate
//! key = '/path/to/private/key/pemfile'
//! ```
//!
//! ## Example
//!
//! ```rust
//! use epp_client::config::CONFIG;
//!
//! fn main() {
//!     // Get configuration for the relevant registry section
//!     let registry = CONFIG.registry("verisign").unwrap();
//!
//!     // Get EPP host name and port no.
//!     let remote = registry.connection_details();
//!
//!     // Get username and password
//!     let credentials = registry.credentials();
//!
//!     // Get EPP service extensions
//!     let service_extensions = registry.ext_uris().unwrap();
//!
//!     // Get client certificate and private key
//!     let tls = registry.tls_files().unwrap();
//! }
//! ```

use confy;
use lazy_static::lazy_static;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default;
use std::{fs, io};

lazy_static! {
    pub static ref CONFIG: EppClientConfig = match confy::load("epp-client") {
        Ok(cfg) => cfg,
        Err(e) => panic!("Config read error: {}", e),
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientTlsFiles {
    cert_chain: String,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConnection {
    host: String,
    port: u16,
    username: String,
    password: String,
    ext_uris: Option<Vec<String>>,
    tls_files: Option<EppClientTlsFiles>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConfig {
    pub registry: HashMap<String, EppClientConnection>,
}

impl default::Default for EppClientConfig {
    fn default() -> Self {
        let mut registries: HashMap<String, EppClientConnection> = HashMap::new();
        let registrar = EppClientConnection {
            host: "epphost".to_string(),
            port: 700,
            username: "username".to_string(),
            password: "password".to_string(),
            ext_uris: Some(vec![]),
            tls_files: Some(EppClientTlsFiles {
                cert_chain: "/path/to/certificate/chain/pemfile".to_string(),
                key: "/path/to/private/key/pemfile".to_string(),
            }),
        };
        registries.insert("hexonet".to_string(), registrar);
        Self {
            registry: registries,
        }
    }
}

impl EppClientConnection {
    pub fn connection_details(&self) -> (String, u16) {
        (self.host.to_string(), self.port)
    }
    pub fn credentials(&self) -> (String, String) {
        (self.username.to_string(), self.password.to_string())
    }
    pub fn ext_uris(&self) -> Option<&Vec<String>> {
        self.ext_uris.as_ref()
    }
    pub fn tls_files(&self) -> Option<(Vec<Certificate>, PrivateKey)> {
        let certificates = self.client_certificate();
        let key = self.key();

        if certificates == None || key == None {
            None
        } else {
            Some((certificates.unwrap(), key.unwrap()))
        }
    }
    fn client_certificate(&self) -> Option<Vec<Certificate>> {
        match &self.tls_files {
            Some(tls) => Some(
                rustls_pemfile::certs(&mut io::BufReader::new(
                    fs::File::open(tls.cert_chain.to_string()).unwrap(),
                ))
                .unwrap()
                .iter()
                .map(|v| Certificate(v.clone()))
                .collect(),
            ),
            None => None,
        }
    }
    fn key(&self) -> Option<PrivateKey> {
        match &self.tls_files {
            Some(tls) => Some(rustls::PrivateKey(
                rustls_pemfile::rsa_private_keys(&mut io::BufReader::new(
                    fs::File::open(tls.key.to_string()).unwrap(),
                ))
                .unwrap()[0]
                    .clone(),
            )),
            None => None,
        }
    }
}

impl EppClientConfig {
    pub fn registry(&self, registry: &str) -> Option<&EppClientConnection> {
        self.registry.get(registry)
    }
}
