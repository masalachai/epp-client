use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppHostCreateResponse = EppObject<CommandResponse<HostCreateResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    pub name: StringValue,
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateResult {
    #[serde(rename = "creData")]
    pub check_data: HostCreateData,
}
