use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppDomainCreateResponse = EppObject<CommandResponse<DomainCreateResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateData {
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    pub name: StringValue,
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    #[serde(rename = "exDate")]
    pub expiry_date: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResult {
    #[serde(rename = "creData")]
    pub create_data: DomainCreateData,
}
