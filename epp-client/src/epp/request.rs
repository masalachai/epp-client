//! Types for EPP requests

pub mod contact;
pub mod domain;
pub mod host;
pub mod message;

use serde::{ser::SerializeStruct, ser::Serializer, Deserialize, Serialize};
use std::error::Error;
use std::time::SystemTime;

use crate::epp::object::{
    ElementName, EmptyTag, EppObject, Extension, Options, ServiceExtension, Services, StringValue,
    StringValueTrait,
};
use crate::epp::xml::{EPP_CONTACT_XMLNS, EPP_DOMAIN_XMLNS, EPP_HOST_XMLNS, EPP_LANG, EPP_VERSION};
use epp_client_macros::*;

/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// without an &lt;extension&gt; tag
pub type Command<T> = CommandWithExtension<T, EmptyTag>;

/// The EPP Hello request
pub type EppHello = EppObject<Hello>;
/// The EPP Login Request
pub type EppLogin = EppObject<Command<Login>>;
/// The EPP Logout request
pub type EppLogout = EppObject<Command<Logout>>;

#[derive(Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "command")]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
pub struct CommandWithExtension<T: ElementName, E: ElementName> {
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub command: T,
    /// The client TRID
    pub extension: Option<Extension<E>>,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<T: ElementName + Serialize, E: ElementName + Serialize> Serialize
    for CommandWithExtension<T, E>
{
    /// Serializes the generic type T to the proper XML tag (set by the `#[element_name(name = <tagname>)]` attribute) for the request
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("command", 3)?;
        state.serialize_field(T::ELEMENT, &self.command)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}

impl<T: ElementName> Command<T> {
    /// Creates a new &lt;command&gt; tag for an EPP document
    pub fn new(command: T, client_tr_id: &str) -> Command<T> {
        Command {
            command,
            extension: None,
            client_tr_id: client_tr_id.to_string_value(),
        }
    }
}

impl<T: ElementName, E: ElementName> CommandWithExtension<T, E> {
    /// Creates a new &lt;command&gt; tag for an EPP document with a containing &lt;extension&gt; tag
    pub fn build(command: T, ext: E, client_tr_id: &str) -> CommandWithExtension<T, E> {
        CommandWithExtension {
            command,
            extension: Some(Extension { data: ext }),
            client_tr_id: client_tr_id.to_string_value(),
        }
    }
}

/// Basic client TRID generation function. Mainly used for testing. Users of the library should use their own clTRID generation function.
pub fn generate_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}:{}", username, timestamp.as_secs()))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "hello")]
/// Type corresponding to the <hello> tag in an EPP XML hello request
pub struct Hello;

impl EppHello {
    /// Creates a new Epp Hello request
    pub fn new() -> EppHello {
        EppObject::build(Hello {})
    }
}

impl Default for EppHello {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "login")]
/// Type corresponding to the &lt;login&gt; tag in an EPP XML login request
pub struct Login {
    /// The username to use for the login
    #[serde(rename(serialize = "clID", deserialize = "clID"))]
    username: StringValue,
    /// The password to use for the login
    #[serde(rename = "pw", default)]
    password: StringValue,
    /// Data under the <options> tag
    options: Options,
    /// Data under the <svcs> tag
    #[serde(rename = "svcs")]
    services: Services,
}

impl EppLogin {
    /// Creates a new EPP Login request
    pub fn new(
        username: &str,
        password: &str,
        ext_uris: &Option<Vec<String>>,
        client_tr_id: &str,
    ) -> EppLogin {
        let ext_uris = ext_uris.as_ref().map(|uris| {
            uris.iter()
                .map(|u| u.to_string_value())
                .collect::<Vec<StringValue>>()
        });

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
                svc_ext: Some(ServiceExtension { ext_uris }),
            },
        };

        EppObject::build(Command::<Login> {
            command: login,
            extension: None,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the <options> tag data
    pub fn options(&mut self, options: Options) {
        self.data.command.options = options;
    }

    /// Sets the <svcs> tag data
    pub fn services(&mut self, services: Services) {
        self.data.command.services = services;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "logout")]
/// Type corresponding to the &lt;logout&gt; tag in an EPP XML logout request
pub struct Logout;

impl EppLogout {
    /// Creates a new EPP Logout request
    pub fn new(client_tr_id: &str) -> EppLogout {
        EppObject::build(Command::<Logout> {
            command: Logout,
            extension: None,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
