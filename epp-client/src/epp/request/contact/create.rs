//! Types for EPP contact create request

use epp_client_macros::*;

use crate::epp::object::data;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for contact <create> command
pub type EppContactCreate = EppObject<Command<ContactCreate>>;

/// Type for elements under the contact <create> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    /// XML namespace for contact commands
    xmlns: String,
    /// Contact <id> tag
    id: StringValue,
    /// Contact <postalInfo> tag
    #[serde(rename = "postalInfo")]
    postal_info: data::PostalInfo,
    /// Contact <voice> tag
    voice: data::Phone,
    /// Contact <fax> tag,
    fax: Option<data::Phone>,
    /// Contact <email> tag
    email: StringValue,
    /// Contact <authInfo> tag
    #[serde(rename = "authInfo")]
    auth_info: data::AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML <create> command for contacts
pub struct ContactCreate {
    /// Data for <create> command for contact
    #[serde(rename = "create")]
    pub contact: Contact,
}

impl EppContactCreate {
    /// Creates a new EppObject for contact create corresponding to the <epp> tag in EPP XML
    pub fn new(
        id: &str,
        email: &str,
        postal_info: data::PostalInfo,
        voice: data::Phone,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppContactCreate {
        let contact_create = ContactCreate {
            contact: Contact {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.to_string_value(),
                postal_info: postal_info,
                voice: voice,
                fax: None,
                email: email.to_string_value(),
                auth_info: data::AuthInfo::new(auth_password),
            },
        };

        EppObject::build(Command::<ContactCreate> {
            command: contact_create,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the <fax> data for the request
    pub fn set_fax(&mut self, fax: data::Phone) {
        self.data.command.contact.fax = Some(fax);
    }
}
