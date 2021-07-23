use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppContactDelete = EppObject<Command<ContactDelete>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDeleteData {
    pub xmlns: String,
    pub id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
pub struct ContactDelete {
    #[serde(rename = "delete")]
    contact: ContactDeleteData,
}

impl EppContactDelete {
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
