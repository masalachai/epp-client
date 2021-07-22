use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainCheck = EppObject<Command<DomainCheck>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    pub xmlns: String,
    #[serde(rename = "name")]
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "checkr")]
pub struct DomainCheck {
    #[serde(rename = "check")]
    list: DomainList,
}

impl EppDomainCheck {
    pub fn new(domains: Vec<&str>, client_tr_id: &str) -> EppDomainCheck {
        let domains = domains
            .iter()
            .filter_map(|d| Some(d.to_string_value()))
            .collect::<Vec<StringValue>>();

        let domain_check = DomainCheck {
            list: DomainList {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                domains: domains,
            },
        };

        EppObject::build(Command::<DomainCheck> {
            command: domain_check,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
