use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    common::{NoExtension, Options, ServiceExtension, Services, StringValue},
    contact, domain, host,
    request::{Command, Transaction, EPP_LANG, EPP_VERSION},
};

impl Transaction<NoExtension> for Login {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

impl Login {
    pub fn new(username: &str, password: &str, ext_uris: Option<Vec<String>>) -> Self {
        let ext_uris = ext_uris.map(|uris| uris.iter().map(|u| u.as_str().into()).collect());

        Self {
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
        }
    }

    /// Sets the <options> tag data
    pub fn options(&mut self, options: Options) {
        self.options = options;
    }

    /// Sets the <svcs> tag data
    pub fn services(&mut self, services: Services) {
        self.services = services;
    }
}

impl Command for Login {
    type Response = ();
    const COMMAND: &'static str = "login";
}

#[cfg(test)]
mod tests {
    use super::Login;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn login() {
        let xml = get_xml("response/login.xml").unwrap();
        let object = Login::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
