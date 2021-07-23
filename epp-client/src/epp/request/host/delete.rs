use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppHostDelete = EppObject<Command<HostDelete>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostDeleteData {
    xmlns: String,
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
pub struct HostDelete {
    #[serde(rename = "delete")]
    host: HostDeleteData,
}

impl EppHostDelete {
    pub fn new(name: &str, client_tr_id: &str) -> EppHostDelete {
        EppObject::build(Command::<HostDelete> {
            command: HostDelete {
                host: HostDeleteData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
