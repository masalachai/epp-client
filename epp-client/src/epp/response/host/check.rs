//! Types for EPP host check response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML host check response
pub type EppHostCheckResponse = EppObject<CommandResponse<HostCheckResult>>;

/// Type that represents the &lt;name&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheck {
    /// The host name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The host (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub host: HostCheck,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub host_list: Vec<HostCheckDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckResult {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: HostCheckData,
}
