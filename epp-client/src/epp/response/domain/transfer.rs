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

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferResult {
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferData,
}
