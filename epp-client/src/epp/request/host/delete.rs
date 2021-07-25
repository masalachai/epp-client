//! Types for EPP host delete request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for host <delete> command
pub type EppHostDelete = EppObject<Command<HostDelete>>;

/// Type for data under the host <delete> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostDeleteData {
    /// XML namespace for host commands
    xmlns: String,
    /// The host to be deleted
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML <delete> command for hosts
pub struct HostDelete {
    /// The instance holding the data for the host to be deleted
    #[serde(rename = "delete")]
    host: HostDeleteData,
}

impl EppHostDelete {
    /// Creates a new EppObject for host delete corresponding to the <epp> tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppHostDelete {
        EppObject::build(Command::<HostDelete> {
            command: HostDelete {
                host: HostDeleteData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}