pub mod contact;
pub mod domain;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

use crate::epp::command::Command;
use crate::epp::object::{
    ElementName, EppObject, Options, ServiceExtension, Services, StringValue, StringValueTrait,
};
use crate::epp::xml::{EPP_LANG, EPP_VERSION};
use epp_client_macros::*;

pub type EppHello = EppObject<Hello>;
pub type EppLogin = EppObject<Command<Login>>;
pub type EppLogout = EppObject<Command<Logout>>;

pub fn generate_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}:{}", username, timestamp.as_secs()))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[serde(rename = "hello")]
#[element_name(name = "hello")]
pub struct Hello;

impl EppHello {
    pub fn new() -> EppHello {
        EppObject::build(Hello {})
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[serde(rename = "login")]
#[element_name(name = "login")]
pub struct Login {
    #[serde(rename(serialize = "clID", deserialize = "clID"))]
    username: StringValue,
    #[serde(rename = "pw", default)]
    password: StringValue,
    options: Options,
    #[serde(rename = "svcs")]
    services: Services,
}

impl EppLogin {
    pub fn new(username: &str, password: &str, client_tr_id: &str) -> EppLogin {
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

        EppObject::build(Command::<Login> {
            command: login,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn set_options(&mut self, options: Options) {
        self.data.command.options = options;
    }

    pub fn set_services(&mut self, services: Services) {
        self.data.command.services = services;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "logout")]
pub struct Logout;

impl EppLogout {
    pub fn new(client_tr_id: &str) -> EppLogout {
        EppObject::build(Command::<Logout> {
            command: Logout,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
