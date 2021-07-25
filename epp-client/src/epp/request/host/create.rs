//! Types for EPP host create request

use epp_client_macros::*;

use crate::epp::object::data::{Host, HostAddr};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for host <create> command
pub type EppHostCreate = EppObject<Command<HostCreate>>;

/// Type for data under the host <create> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    /// XML namespace for host commands
    xmlns: String,
    /// The name of the host to be created
    pub name: StringValue,
    /// The list of IP addresses for the host
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML <create> command for hosts
pub struct HostCreate {
    /// The instance holding the data for the host to be created
    #[serde(rename = "create")]
    host: HostCreateData,
}

impl EppHostCreate {
    /// Creates a new EppObject for host create corresponding to the <epp> tag in EPP XML
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
