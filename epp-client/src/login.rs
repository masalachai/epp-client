use std::fmt::Debug;

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, EppObject, Options, ServiceExtension, Services, StringValue},
    contact::EPP_CONTACT_XMLNS,
    domain::EPP_DOMAIN_XMLNS,
    host::EPP_HOST_XMLNS,
    request::{Command, EPP_LANG, EPP_VERSION},
    response::EppCommandResponse,
};

/// The EPP Login Request
pub type EppLogin = EppObject<Command<Login>>;

impl EppLogin {
    /// Creates a new EPP Login request
    pub fn new(
        username: &str,
        password: &str,
        ext_uris: &Option<Vec<String>>,
        client_tr_id: &str,
    ) -> EppLogin {
        let ext_uris = ext_uris
            .as_ref()
            .map(|uris| uris.iter().map(|u| u.as_str().into()).collect());

        let login = Login {
            username: username.into(),
            password: password.into(),
            options: Options {
                version: EPP_VERSION.into(),
                lang: EPP_LANG.into(),
            },
            services: Services {
                obj_uris: vec![
                    EPP_HOST_XMLNS.into(),
                    EPP_CONTACT_XMLNS.into(),
                    EPP_DOMAIN_XMLNS.into(),
                ],
                svc_ext: Some(ServiceExtension { ext_uris }),
            },
        };

        EppObject::build(Command::<Login> {
            command: login,
            extension: None,
            client_tr_id: client_tr_id.into(),
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

/// An alias of `EppCommandResponse` received in response to a successful login request
pub type EppLoginResponse = EppCommandResponse;

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
