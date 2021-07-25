//! Types for EPP domain delete request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for domain <delete> command
pub type EppDomainDelete = EppObject<Command<DomainDelete>>;

/// Type for <name> element under the domain <delete> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainDeleteData {
    /// XML namespace for domain commands
    xmlns: String,
    /// The domain to be deleted
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML <delete> command for domains
pub struct DomainDelete {
    /// The data under the <delete> tag for domain deletion
    #[serde(rename = "delete")]
    domain: DomainDeleteData,
}

impl EppDomainDelete {
    /// Creates a new EppObject for domain delete corresponding to the <epp> tag in EPP XML
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
