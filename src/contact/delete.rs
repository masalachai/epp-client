//! Types for EPP contact delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl Transaction<NoExtension> for ContactDelete {}

impl Command for ContactDelete {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Debug)]
pub struct ContactDeleteRequestData {
    /// XML namespace for the &lt;delete&gt; command for contacts
    #[serde(rename = "xmlns:contact")]
    xmlns: &'static str,
    /// The id of the contact to be deleted
    #[serde(rename = "contact:id")]
    id: StringValue,
}

#[derive(Serialize, Debug)]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDelete {
    #[serde(rename = "contact:delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequestData,
}

impl ContactDelete {
    pub fn new(id: &str) -> ContactDelete {
        Self {
            contact: ContactDeleteRequestData {
                xmlns: XMLNS,
                id: id.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContactDelete;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/contact/delete.xml").unwrap();

        let object = ContactDelete::new("eppdev-contact-3");

        let serialized =
            <ContactDelete as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/contact/delete.xml").unwrap();
        let object =
            <ContactDelete as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
