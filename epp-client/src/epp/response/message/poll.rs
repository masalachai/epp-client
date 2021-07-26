use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;
use serde::{Deserialize, Serialize};

pub type EppMessagePollResponse = EppObject<CommandResponse<MessagePollResult>>;

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
    pub requested_at: StringValue,
    #[serde(rename = "acID")]
    pub ack_id: StringValue,
    #[serde(rename = "acDate")]
    pub ack_by: StringValue,
    #[serde(rename = "exDate")]
    pub expiry_date: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePollResult {
    #[serde(rename = "trnData")]
    pub message_data: MessageDomainTransferData,
}
