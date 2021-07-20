use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use serde::{Deserialize, Serialize};

const EPP_CONTACT_XMLNS: &str = "urn:ietf:params:xml:ns:contact-1.0";

pub type EppContactCheck = EppObject<Command<ContactCheck>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactList {
    pub xmlns: String,
    #[serde(rename = "id")]
    pub contact_ids: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheck {
    #[serde(rename = "check")]
    list: ContactList,
}

impl ElementName for ContactCheck {
    fn element_name(&self) -> &'static str {
        "check"
    }
}

impl EppContactCheck {
    pub fn new(contact_ids: Vec<&str>, client_tr_id: &str) -> EppContactCheck {
        let contact_ids = contact_ids
            .iter()
            .filter_map(|d| Some(d.to_string_value()))
            .collect::<Vec<StringValue>>();

        let contact_check = ContactCheck {
            list: ContactList {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                contact_ids: contact_ids,
            },
        };

        EppObject::build(Command::<ContactCheck> {
            command: contact_check,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
