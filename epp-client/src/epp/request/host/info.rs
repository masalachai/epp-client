use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppHostInfo = EppObject<Command<HostInfo>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoData {
    xmlns: String,
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
pub struct HostInfo {
    #[serde(rename = "info")]
    info: HostInfoData,
}

impl EppHostInfo {
    pub fn new(name: &str, client_tr_id: &str) -> EppHostInfo {
        EppObject::build(Command::<HostInfo> {
            command: HostInfo {
                info: HostInfoData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
