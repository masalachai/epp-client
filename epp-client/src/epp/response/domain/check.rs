use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppDomainCheckResponse = EppObject<CommandResponse<DomainCheckResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheck {
    #[serde(rename = "$value")]
    pub name: StringValue,
    #[serde(rename = "avail")]
    pub available: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckDataItem {
    #[serde(rename = "name")]
    pub domain: DomainCheck,
    pub reason: Option<StringValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckData {
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckDataItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResult {
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckData,
}
