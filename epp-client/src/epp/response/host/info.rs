//! Types for EPP host info response

use serde::{Deserialize, Serialize};

use crate::epp::object::data::{HostAddr, HostStatus};
use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML host info response
pub type EppHostInfoResponse = EppObject<CommandResponse<HostInfoResult>>;

/// Type that represents the &lt;infData&gt; tag for host info response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoResult {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: HostInfoData,
}
