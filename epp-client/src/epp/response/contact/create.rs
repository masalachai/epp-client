use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppContactCreateResponse = EppObject<CommandResponse<ContactCreateResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateData {
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    pub id: StringValue,
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateResult {
    #[serde(rename = "creData")]
    pub check_data: ContactCreateData,
}
