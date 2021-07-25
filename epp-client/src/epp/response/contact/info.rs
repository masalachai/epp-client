use serde::{Deserialize, Serialize};

use crate::epp::object::data::{AuthInfo, ContactStatus, Phone, PostalInfo};
use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppContactInfoResponse = EppObject<CommandResponse<ContactInfoResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoData {
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    pub id: StringValue,
    pub roid: StringValue,
    #[serde(rename = "status")]
    pub statuses: Vec<ContactStatus>,
    #[serde(rename = "postalInfo")]
    pub postal_info: PostalInfo,
    pub voice: Phone,
    pub fax: Option<Phone>,
    pub email: StringValue,
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    #[serde(rename = "authInfo")]
    pub auth_info: Option<AuthInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoResult {
    #[serde(rename = "infData")]
    pub info_data: ContactInfoData,
}
