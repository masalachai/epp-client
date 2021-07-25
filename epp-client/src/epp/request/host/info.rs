//! Types for EPP host info request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for host <info> command
pub type EppHostInfo = EppObject<Command<HostInfo>>;

/// Type for data under the host <info> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoData {
    /// XML namespace for host commands
    xmlns: String,
    /// The name of the host to be queried
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML <info> command for hosts
pub struct HostInfo {
    /// The instance holding the data for the host query
    #[serde(rename = "info")]
    info: HostInfoData,
}

impl EppHostInfo {
    /// Creates a new EppObject for host info corresponding to the <epp> tag in EPP XML
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
