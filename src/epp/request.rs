pub mod domain;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

pub use crate::epp::command::Command;
use crate::epp::object::{
    ElementName, EppObject, Options, ServiceExtension, Services, StringValue, StringValueTrait,
};
use crate::epp::xml::{EPP_LANG, EPP_VERSION};

pub type EppHello = EppObject<Hello>;
pub type EppLogin = EppObject<Command<Login>>;
pub type EppLogout = EppObject<Command<Logout>>;

pub fn generate_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}:{}", username, timestamp.as_secs()))
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "hello")]
pub struct Hello;

impl ElementName for Hello {
    fn element_name(&self) -> &'static str {
        "hello"
    }
}

impl Hello {
    pub fn epp_new() -> EppHello {
        EppObject::new(Hello {})
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "login")]
pub struct Login {
    #[serde(rename(serialize = "clID", deserialize = "clID"))]
    username: StringValue,
    #[serde(rename = "pw", default)]
    password: StringValue,
    options: Options,
    #[serde(rename = "svcs")]
    services: Services,
}

impl ElementName for Login {
    fn element_name(&self) -> &'static str {
        "login"
    }
}

impl Login {
    pub fn epp_new(username: &str, password: &str, client_tr_id: &str) -> EppLogin {
        let login = Login {
            username: username.to_string_value(),
            password: password.to_string_value(),
            options: Options {
                version: EPP_VERSION.to_string_value(),
                lang: EPP_LANG.to_string_value(),
            },
            services: Services {
                obj_uris: vec![
                    "urn:ietf:params:xml:ns:host-1.0".to_string_value(),
                    "urn:ietf:params:xml:ns:contact-1.0".to_string_value(),
                    "urn:ietf:params:xml:ns:domain-1.0".to_string_value(),
                ],
                svc_ext: Some(ServiceExtension {
                    ext_uris: Some(vec![
                        "http://schema.ispapi.net/epp/xml/keyvalue-1.0".to_string_value()
                    ]),
                }),
            },
        };

        EppObject::new(Command::<Login> {
            command: login,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    pub fn set_services(&mut self, services: Services) {
        self.services = services;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Logout;

impl Logout {
    pub fn epp_new(client_tr_id: &str) -> EppLogout {
        EppObject::new(Command::<Logout> {
            command: Logout,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}

impl ElementName for Logout {
    fn element_name(&self) -> &'static str {
        "logout"
    }
}
