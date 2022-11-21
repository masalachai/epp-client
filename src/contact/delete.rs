//! Types for EPP contact delete request

use instant_xml::ToXml;

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for ContactDelete<'a> {}

impl<'a> Command for ContactDelete<'a> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(XMLNS))]
pub struct ContactDeleteRequest<'a> {
    /// The id of the contact to be deleted
    id: &'a str,
}

/// The &lt;delete&gt; type for the contact delete EPP command
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(EPP_XMLNS))]
pub struct ContactDelete<'a> {
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequest<'a>,
}

impl<'a> ContactDelete<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            contact: ContactDeleteRequest { id },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContactDelete;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = ContactDelete::new("eppdev-contact-3");
        assert_serialized("request/contact/delete.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<ContactDelete>("response/contact/delete.xml");
        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
