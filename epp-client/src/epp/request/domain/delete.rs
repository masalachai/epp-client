use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainDelete = EppObject<Command<DomainDelete>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainDeleteData {
    pub xmlns: String,
    pub name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
pub struct DomainDelete {
    #[serde(rename = "delete")]
    domain: DomainDeleteData,
}

impl EppDomainDelete {
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainDelete {
        EppObject::build(Command::<DomainDelete> {
            command: DomainDelete {
                domain: DomainDeleteData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
