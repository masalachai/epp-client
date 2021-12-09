//! Types for EPP host info request

use super::XMLNS;
use crate::common::{HostAddr, HostStatus, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for HostInfo {}

impl Command for HostInfo {
    type Response = HostInfoResponse;
    const COMMAND: &'static str = "info";
}

impl HostInfo {
    pub fn new(name: &str) -> Self {
        Self {
            info: HostInfoRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
            },
        }
    }
}

// Request

/// Type for data under the host &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct HostInfoRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host to be queried
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;info&gt; command for hosts
pub struct HostInfo {
    /// The instance holding the data for the host query
    #[serde(rename = "host:info", alias = "info")]
    info: HostInfoRequestData,
}

// Response

/// Type that represents the &lt;infData&gt; tag for host info response
#[derive(Deserialize, Debug)]
pub struct HostInfoResponseData {
    /// The host name
    pub name: StringValue,
    /// The host ROID
    pub roid: StringValue,
    /// The list of host statuses
    #[serde(rename = "status")]
    pub statuses: Vec<HostStatus>,
    /// The list of host IP addresses
    #[serde(rename = "addr")]
    pub addresses: Vec<HostAddr>,
    /// The epp user to whom the host belongs
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// THe epp user that created the host
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The epp user that last updated the host
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    /// The host last update date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    /// The host transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
}

/// Type that represents the &lt;resData&gt; tag for host info response
#[derive(Deserialize, Debug)]
pub struct HostInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: HostInfoResponseData,
}
