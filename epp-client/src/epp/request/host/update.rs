//! Types for EPP host update request

use epp_client_macros::*;

use crate::epp::object::data::{HostAddr, HostStatus};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for host <update> command
pub type EppHostUpdate = EppObject<Command<HostUpdate>>;

/// Type for data under the <chg> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostChangeInfo {
    /// The new name for the host
    pub name: StringValue,
}

/// Type for data under the <add> and <rem> tags
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddRemove {
    /// The IP addresses to be added to or removed from the host
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
    /// The statuses to be added to or removed from the host
    #[serde(rename = "status")]
    pub statuses: Option<Vec<HostStatus>>,
}

/// Type for data under the host <update> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostUpdateData {
    /// XML namespace for host commands
    xmlns: String,
    /// The name of the host
    name: StringValue,
    /// The IP addresses and statuses to be added to the host
    add: Option<HostAddRemove>,
    /// The IP addresses and statuses to be removed from the host
    #[serde(rename = "rem")]
    remove: Option<HostAddRemove>,
    /// The host details that need to be updated
    #[serde(rename = "chg")]
    change_info: Option<HostChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML <update> command for hosts
pub struct HostUpdate {
    /// The instance holding the data for the host to be updated
    #[serde(rename = "update")]
    host: HostUpdateData,
}

impl EppHostUpdate {
    /// Creates a new EppObject for host update corresponding to the <epp> tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppHostUpdate {
        EppObject::build(Command::<HostUpdate> {
            command: HostUpdate {
                host: HostUpdateData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the data for the <chg> element of the host update
    pub fn info(&mut self, info: HostChangeInfo) {
        self.data.command.host.change_info = Some(info);
    }

    /// Sets the data for the <add> element of the host update
    pub fn add(&mut self, add: HostAddRemove) {
        self.data.command.host.add = Some(add);
    }

    /// Sets the data for the <rem> element of the host update
    pub fn remove(&mut self, remove: HostAddRemove) {
        self.data.command.host.remove = Some(remove);
    }
}
