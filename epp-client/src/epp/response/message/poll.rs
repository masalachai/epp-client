use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;
use serde::{Deserialize, Serialize};

pub type EppMessagePollResponse = EppObject<CommandResponse<MessageDomainTransferData>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDomainTransferData {
    #[serde(rename = "xmlns:obj")]
    xmlns: String,
    pub name: StringValue,
    #[serde(rename = "trStatus")]
    pub transfer_status: StringValue,
    #[serde(rename = "reID")]
    pub requester_id: StringValue,
    #[serde(rename = "reDate")]
    pub request_date: StringValue,
    #[serde(rename = "acID")]
    pub responder_id: StringValue,
    #[serde(rename = "acDate")]
    pub respond_by_date: StringValue,
    #[serde(rename = "exDate")]
    pub expiry_date: StringValue,
}
