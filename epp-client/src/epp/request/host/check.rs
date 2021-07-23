use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppHostCheck = EppObject<Command<HostCheck>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostList {
    xmlns: String,
    #[serde(rename = "name")]
    pub hosts: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
pub struct HostCheck {
    #[serde(rename = "check")]
    list: HostList,
}

impl EppHostCheck {
    pub fn new(hosts: Vec<&str>, client_tr_id: &str) -> EppHostCheck {
        let hosts = hosts
            .iter()
            .filter_map(|d| Some(d.to_string_value()))
            .collect::<Vec<StringValue>>();

        let host_check = HostCheck {
            list: HostList {
                xmlns: EPP_HOST_XMLNS.to_string(),
                hosts: hosts,
            },
        };

        EppObject::build(Command::<HostCheck> {
            command: host_check,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
