use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

use crate::epp::object::{
    EppObject, Options, ServiceExtension, Services, StringValue, StringValueTrait,
};
use crate::epp::xml::{EPP_LANG, EPP_VERSION, EPP_XMLNS, EPP_XMLNS_XSI, EPP_XSI_SCHEMA_LOCATION};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RequestType {
    Hello,
    #[serde(rename = "command")]
    CommandLogin {
        login: Login,
        #[serde(rename = "clTRID")]
        client_tr_id: StringValue,
    },
    #[serde(rename = "command")]
    CommandLogout {
        logout: Logout,
        #[serde(rename = "clTRID")]
        client_tr_id: StringValue,
    },
}

impl<RequestType> EppObject<RequestType> {
    pub fn generate_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(format!("{}:{}", username, timestamp.as_secs()))
    }
}

pub type EppRequest = EppObject<RequestType>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct Hello;

impl Hello {
    pub fn new() -> EppRequest {
        EppRequest::new(RequestType::Hello)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Command {
    login: Login,
    #[serde(rename = "clTRID")]
    client_tr_id: StringValue,
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

impl Login {
    pub fn new(username: &str, password: &str, client_tr_id: &str) -> EppRequest {
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

        EppRequest::new(RequestType::CommandLogin {
            login: login,
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
    pub fn new(client_tr_id: &str) -> EppRequest {
        EppRequest::new(RequestType::CommandLogout {
            logout: Logout,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
