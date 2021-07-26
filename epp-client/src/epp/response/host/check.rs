//! Types for EPP host check response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the <epp> tag for the EPP XML host check response
pub type EppHostCheckResponse = EppObject<CommandResponse<HostCheckResult>>;

/// Type that represents the <name> tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheck {
    /// The host name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The host (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the <cd> tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckDataItem {
    /// Data under the <name> tag
    #[serde(rename = "name")]
    pub host: HostCheck,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the <chkData> tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// XML schema location for host response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// Data under the <cd> tag
    #[serde(rename = "cd")]
    pub host_list: Vec<HostCheckDataItem>,
}

/// Type that represents the <resData> tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckResult {
    /// Data under the <chkData> tag
    #[serde(rename = "chkData")]
    pub check_data: HostCheckData,
}
