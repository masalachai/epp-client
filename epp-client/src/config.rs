use confy;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default;

lazy_static! {
    pub static ref CONFIG: EppClientConfig = match confy::load("epp-client") {
        Ok(cfg) => cfg,
        Err(e) => panic!("Config read error: {}", e),
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConnection {
    host: String,
    port: u16,
    username: String,
    password: String,
    ext_uris: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EppClientConfig {
    pub servers: HashMap<String, EppClientConnection>,
}

impl default::Default for EppClientConfig {
    fn default() -> Self {
        let servers: HashMap<String, EppClientConnection> = HashMap::new();
        Self { servers: servers }
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
}

impl EppClientConfig {
    pub fn registry(&self, registry: &str) -> Option<&EppClientConnection> {
        self.servers.get(registry)
    }
}
