//! Types for EPP contact delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for ContactDelete<'a> {}

impl<'a> Command for ContactDelete<'a> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Debug)]
pub struct ContactDeleteRequestData<'a> {
    /// XML namespace for the &lt;delete&gt; command for contacts
    #[serde(rename = "xmlns:contact")]
    xmlns: &'a str,
    /// The id of the contact to be deleted
    #[serde(rename = "contact:id")]
    id: StringValue<'a>,
}

#[derive(Serialize, Debug)]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDelete<'a> {
    #[serde(rename = "contact:delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequestData<'a>,
}

impl<'a> ContactDelete<'a> {
    pub fn new(id: &'a str) -> Self {
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
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
