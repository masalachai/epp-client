use serde::{Deserialize, Serialize};

use crate::epp::object::data::{HostAddr, HostStatus};
use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

pub type EppHostInfoResponse = EppObject<CommandResponse<HostInfoResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoData {
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    pub name: StringValue,
    pub roid: StringValue,
    #[serde(rename = "status")]
    pub statuses: Vec<HostStatus>,
    #[serde(rename = "addr")]
    pub addresses: Vec<HostAddr>,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoResult {
    #[serde(rename = "infData")]
    pub info_data: HostInfoData,
}
