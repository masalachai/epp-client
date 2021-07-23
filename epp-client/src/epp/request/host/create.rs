use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::{Host, HostAddr};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppHostCreate = EppObject<Command<HostCreate>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    xmlns: String,
    pub name: StringValue,
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
pub struct HostCreate {
    #[serde(rename = "create")]
    host: HostCreateData,
}

impl EppHostCreate {
    pub fn new(host: Host, client_tr_id: &str) -> EppHostCreate {
        let host_create = HostCreate {
            host: HostCreateData {
                xmlns: EPP_HOST_XMLNS.to_string(),
                name: host.name,
                addresses: host.addresses,
            },
        };

        EppObject::build(Command::<HostCreate> {
            command: host_create,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
