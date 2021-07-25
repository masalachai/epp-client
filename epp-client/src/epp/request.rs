pub mod contact;
pub mod domain;
pub mod host;
pub mod message;

use serde::{ser::SerializeStruct, ser::Serializer, Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

use crate::epp::object::{
    ElementName, EppObject, Options, ServiceExtension, Services, StringValue, StringValueTrait,
};
use crate::epp::xml::{EPP_CONTACT_XMLNS, EPP_DOMAIN_XMLNS, EPP_HOST_XMLNS, EPP_LANG, EPP_VERSION};
use epp_client_macros::*;

pub type EppHello = EppObject<Hello>;
pub type EppLogin = EppObject<Command<Login>>;
pub type EppLogout = EppObject<Command<Logout>>;

#[derive(Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "command")]
pub struct Command<T: ElementName> {
    pub command: T,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<T: ElementName + Serialize> Serialize for Command<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let command_name = self.command.element_name();
        let mut state = serializer.serialize_struct("command", 2)?;
        state.serialize_field(command_name, &self.command)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}

pub fn generate_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}:{}", username, timestamp.as_secs()))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "hello")]
pub struct Hello;

impl EppHello {
    pub fn new() -> EppHello {
        EppObject::build(Hello {})
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
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
    pub fn new(
        username: &str,
        password: &str,
        ext_uris: &Option<Vec<String>>,
        client_tr_id: &str,
    ) -> EppLogin {
        let ext_uris = match ext_uris {
            Some(uris) => Some(
                uris.iter()
                    .map(|u| u.to_string_value())
                    .collect::<Vec<StringValue>>(),
            ),
            None => None,
        };

        let login = Login {
            username: username.to_string_value(),
            password: password.to_string_value(),
            options: Options {
                version: EPP_VERSION.to_string_value(),
                lang: EPP_LANG.to_string_value(),
            },
            services: Services {
                obj_uris: vec![
                    EPP_HOST_XMLNS.to_string_value(),
                    EPP_CONTACT_XMLNS.to_string_value(),
                    EPP_DOMAIN_XMLNS.to_string_value(),
                ],
                svc_ext: Some(ServiceExtension { ext_uris: ext_uris }),
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
