use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppContactCreate = EppObject<Command<ContactCreate>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    xmlns: String,
    id: StringValue,
    #[serde(rename = "postalInfo")]
    postal_info: data::PostalInfo,
    voice: data::Phone,
    fax: Option<data::Phone>,
    email: StringValue,
    #[serde(rename = "authInfo")]
    auth_info: data::AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
pub struct ContactCreate {
    #[serde(rename = "create")]
    pub contact: Contact,
}

impl EppContactCreate {
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

    pub fn set_fax(&mut self, fax: data::Phone) {
        self.data.command.contact.fax = Some(fax);
    }
}
