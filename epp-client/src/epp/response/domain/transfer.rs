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
    name: StringValue,
    #[serde(rename = "trStatus")]
    transfer_status: StringValue,
    #[serde(rename = "reID")]
    requester_id: StringValue,
    #[serde(rename = "reDate")]
    request_date: StringValue,
    #[serde(rename = "acID")]
    responder_id: StringValue,
    #[serde(rename = "acDate")]
    respond_by_date: StringValue,
    #[serde(rename = "exDate")]
    expiry_date: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferResult {
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferData,
}
