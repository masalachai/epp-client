//! Types for EPP domain info request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for domain <info> command
pub type EppDomainInfo = EppObject<Command<DomainInfo>>;

/// Type for data under the <name> element tag for the domain <info> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    /// The hosts attribute. Default value is "all"
    hosts: String,
    /// The name of the domain
    #[serde(rename = "$value")]
    name: String,
}

/// Type for <name> element under the domain <info> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoData {
    /// XML namespace for domain commands
    xmlns: String,
    /// The data for the domain to be queried
    #[serde(rename = "name")]
    domain: Domain,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML <info> command for domains
pub struct DomainInfo {
    /// The data under the <info> tag for domain info
    #[serde(rename = "info")]
    info: DomainInfoData,
}

impl EppDomainInfo {
    /// Creates a new EppObject for domain info corresponding to the <epp> tag in EPP XML
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
