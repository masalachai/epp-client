use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppDomainRenewResponse = EppObject<CommandResponse<DomainRenewResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewData {
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    pub name: StringValue,
    #[serde(rename = "exDate")]
    pub expiry_date: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewResult {
    #[serde(rename = "renData")]
    pub renew_data: DomainRenewData,
}
