//! Types for EPP domain check request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for domain <check> command
pub type EppDomainCheck = EppObject<Command<DomainCheck>>;

/// Type for <name> elements under the domain <check> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    /// XML namespace for domain commands
    pub xmlns: String,
    #[serde(rename = "name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML <check> command for domains
pub struct DomainCheck {
    /// The object holding the list of domains to be checked
    #[serde(rename = "check")]
    list: DomainList,
}

impl EppDomainCheck {
    /// Creates a new EppObject for domain check corresponding to the <epp> tag in EPP XML
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
