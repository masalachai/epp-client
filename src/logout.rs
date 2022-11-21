use std::fmt::Debug;

use instant_xml::{FromXml, ToXml};

use crate::{
    common::{NoExtension, EPP_XMLNS},
    request::{Command, Transaction},
};

impl Transaction<NoExtension> for Logout {}

impl Command for Logout {
    type Response = ();
    const COMMAND: &'static str = "logout";
}

#[derive(Debug, Eq, FromXml, PartialEq, ToXml)]
/// Type corresponding to the &lt;logout&gt; tag in an EPP XML logout request
#[xml(rename = "logout", ns(EPP_XMLNS))]
pub struct Logout;

#[cfg(test)]
mod tests {
    use super::Logout;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SVTRID};

    #[test]
    fn command() {
        let object = Logout;
        assert_serialized("request/logout.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<Logout>("response/logout.xml");

        assert_eq!(
            object.result.code,
            ResultCode::CommandCompletedSuccessfullyEndingSession
        );
        assert_eq!(
            object.result.message,
            "Command completed successfully; ending session"
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
