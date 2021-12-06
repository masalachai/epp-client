use std::fmt::Debug;

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension, Options, ServiceExtension, Services, StringValue},
    contact, domain, host,
    request::{EppExtension, Transaction, EPP_LANG, EPP_VERSION},
    response::ResponseStatus,
};

#[derive(Debug)]
pub struct Login<E> {
    request: LoginRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for Login<E> {
    type Input = LoginRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> Login<E> {
    pub fn new(
        username: &str,
        password: &str,
        ext_uris: Option<Vec<String>>,
    ) -> Login<NoExtension> {
        let ext_uris = ext_uris.map(|uris| uris.iter().map(|u| u.as_str().into()).collect());

        Login {
            request: LoginRequest {
                username: username.into(),
                password: password.into(),
                options: Options {
                    version: EPP_VERSION.into(),
                    lang: EPP_LANG.into(),
                },
                services: Services {
                    obj_uris: vec![
                        host::XMLNS.into(),
                        contact::XMLNS.into(),
                        domain::XMLNS.into(),
                    ],
                    svc_ext: Some(ServiceExtension { ext_uris }),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> Login<F> {
        Login {
            request: self.request,
            extension: Some(extension),
        }
    }

    /// Sets the <options> tag data
    pub fn options(&mut self, options: Options) {
        self.request.options = options;
    }

    /// Sets the <svcs> tag data
    pub fn services(&mut self, services: Services) {
        self.request.services = services;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "login")]
/// Type corresponding to the &lt;login&gt; tag in an EPP XML login request
pub struct LoginRequest {
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
