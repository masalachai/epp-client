use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppContactCreateResponse = EppObject<CommandResponse<ContactCreateResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateData {
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    pub id: StringValue,
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateResult {
    #[serde(rename = "creData")]
    pub create_data: ContactCreateData,
}
