use std::fmt::Debug;

use serde::Serialize;

use crate::{
    common::{NoExtension, Options, ServiceExtension, Services, StringValue},
    contact, domain, host,
    request::{Command, Transaction, EPP_LANG, EPP_VERSION},
};

impl<'a> Transaction<NoExtension> for Login<'a> {}

#[derive(Serialize, Debug, Eq, PartialEq)]
/// Type corresponding to the &lt;login&gt; tag in an EPP XML login request
pub struct Login<'a> {
    /// The username to use for the login
    #[serde(rename(serialize = "clID", deserialize = "clID"))]
    username: StringValue<'a>,
    /// The password to use for the login
    #[serde(rename = "pw", default)]
    password: StringValue<'a>,
    /// A new password which should be set
    #[serde(rename = "newPW", default, skip_serializing_if = "Option::is_none")]
    new_password: Option<StringValue<'a>>,
    /// Data under the <options> tag
    options: Options<'a>,
    /// Data under the <svcs> tag
    #[serde(rename = "svcs")]
    services: Services<'a>,
}

impl<'a> Login<'a> {
    pub fn new(
        username: &'a str,
        password: &'a str,
        new_password: Option<&'a str>,
        ext_uris: Option<&'_ [&'a str]>,
    ) -> Self {
        let ext_uris = ext_uris.map(|uris| uris.iter().map(|&u| u.into()).collect());

        Self {
            username: username.into(),
            password: password.into(),
            new_password: new_password.map(Into::into),
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
    pub fn options(&mut self, options: Options<'a>) {
        self.options = options;
    }

    /// Sets the <svcs> tag data
    pub fn services(&mut self, services: Services<'a>) {
        self.services = services;
    }
}

impl<'a> Command for Login<'a> {
    type Response = ();
    const COMMAND: &'static str = "login";
}

#[cfg(test)]
mod tests {
    use super::Login;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let ext_uris = Some(&["http://schema.ispapi.net/epp/xml/keyvalue-1.0"][..]);
        let object = Login::new("username", "password", Some("new-password"), ext_uris);
        assert_serialized("request/login.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<Login>("response/login.xml");
        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
