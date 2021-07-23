use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppDomainCreateResponse = EppObject<CommandResponse<DomainCreateResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateData {
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    name: StringValue,
    #[serde(rename = "crDate")]
    created_at: StringValue,
    #[serde(rename = "exDate")]
    expiry_date: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResult {
    #[serde(rename = "creData")]
    pub create_data: DomainCreateData,
}
