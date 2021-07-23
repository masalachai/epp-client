use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainInfo = EppObject<Command<DomainInfo>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    hosts: String,
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoData {
    xmlns: String,
    #[serde(rename = "name")]
    domain: Domain,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
pub struct DomainInfo {
    #[serde(rename = "info")]
    info: DomainInfoData,
}

impl EppDomainInfo {
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainInfo {
        EppObject::build(Command::<DomainInfo> {
            command: DomainInfo {
                info: DomainInfoData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domain: Domain {
                        hosts: "all".to_string(),
                        name: name.to_string(),
                    },
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
