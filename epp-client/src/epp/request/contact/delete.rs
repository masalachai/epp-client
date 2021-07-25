//! Types for EPP contact delete request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type for the <epp> request for contact <delete> command
pub type EppContactDelete = EppObject<Command<ContactDelete>>;

/// Type containing the data for the <delete> tag for contacts
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDeleteData {
    /// XML namespace for the <delete> command for contacts
    xmlns: String,
    /// The id of the contact to be deleted
    id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// The <delete> type for the contact delete EPP command
pub struct ContactDelete {
    #[serde(rename = "delete")]
    /// The data for the <delete> tag for a contact delete command
    contact: ContactDeleteData,
}

impl EppContactDelete {
    /// Creates a new EppObject for contact delete corresponding to the <epp> tag in EPP XML
    pub fn new(id: &str, client_tr_id: &str) -> EppContactDelete {
        EppObject::build(Command::<ContactDelete> {
            command: ContactDelete {
                contact: ContactDeleteData {
                    xmlns: EPP_CONTACT_XMLNS.to_string(),
                    id: id.to_string_value(),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
