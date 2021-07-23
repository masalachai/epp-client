use serde::{Deserialize, Serialize};

use crate::epp::object::data::{AuthInfo, DomainContact, DomainNsList, DomainStatus};
use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppDomainInfoResponse = EppObject<CommandResponse<DomainInfoResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheck {
    #[serde(rename = "$value")]
    pub name: StringValue,
    #[serde(rename = "avail")]
    pub available: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckDataItem {
    pub name: DomainCheck,
    pub reason: Option<StringValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoData {
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    pub name: StringValue,
    pub roid: StringValue,
    pub status: Vec<DomainStatus>,
    pub registrant: StringValue,
    #[serde(rename = "contact")]
    pub contacts: Vec<DomainContact>,
    pub ns: Option<DomainNsList>,
    pub host: Option<Vec<StringValue>>,
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    #[serde(rename = "upID")]
    pub updater_id: StringValue,
    #[serde(rename = "upDate")]
    pub updated_at: StringValue,
    #[serde(rename = "exDate")]
    pub expiry_date: StringValue,
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoResult {
    #[serde(rename = "infData")]
    pub check_data: DomainInfoData,
}
