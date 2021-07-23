use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::{HostAddr, HostStatus};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppHostUpdate = EppObject<Command<HostUpdate>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostChangeInfo {
    pub name: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddRemove {
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
    #[serde(rename = "status")]
    pub statuses: Option<Vec<HostStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostUpdateData {
    xmlns: String,
    name: StringValue,
    add: Option<HostAddRemove>,
    #[serde(rename = "rem")]
    remove: Option<HostAddRemove>,
    #[serde(rename = "chg")]
    change_info: Option<HostChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
pub struct HostUpdate {
    #[serde(rename = "update")]
    host: HostUpdateData,
}

impl EppHostUpdate {
    pub fn new(name: &str, client_tr_id: &str) -> EppHostUpdate {
        EppObject::build(Command::<HostUpdate> {
            command: HostUpdate {
                host: HostUpdateData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn info(&mut self, info: HostChangeInfo) {
        self.data.command.host.change_info = Some(info);
    }

    pub fn add(&mut self, add: HostAddRemove) {
        self.data.command.host.add = Some(add);
    }

    pub fn remove(&mut self, remove: HostAddRemove) {
        self.data.command.host.remove = Some(remove);
    }
}
