use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppHostCheckResponse = EppObject<CommandResponse<HostCheckResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheck {
    #[serde(rename = "$value")]
    pub name: StringValue,
    #[serde(rename = "avail")]
    pub available: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckDataItem {
    #[serde(rename = "name")]
    pub host: HostCheck,
    pub reason: Option<StringValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckData {
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    #[serde(rename = "cd")]
    pub host_list: Vec<HostCheckDataItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckResult {
    #[serde(rename = "chkData")]
    pub check_data: HostCheckData,
}
