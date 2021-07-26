use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;
use crate::epp::response::EppCommandResponse;

pub type EppDomainTransferRequestResponse = EppObject<CommandResponse<DomainTransferResult>>;
pub type EppDomainTransferApproveResponse = EppCommandResponse;
pub type EppDomainTransferRejectResponse = EppCommandResponse;
pub type EppDomainTransferCancelResponse = EppCommandResponse;
pub type EppDomainTransferQueryResponse = EppObject<CommandResponse<DomainTransferResult>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferData {
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
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
pub struct DomainTransferResult {
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferData,
}
