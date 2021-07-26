//! Types for EPP domain transfer response
use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;
use crate::epp::response::EppCommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer request response
pub type EppDomainTransferRequestResponse = EppObject<CommandResponse<DomainTransferResult>>;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer approval response
pub type EppDomainTransferApproveResponse = EppCommandResponse;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer rejection response
pub type EppDomainTransferRejectResponse = EppCommandResponse;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer cancellation response
pub type EppDomainTransferCancelResponse = EppCommandResponse;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer query response
pub type EppDomainTransferQueryResponse = EppObject<CommandResponse<DomainTransferResult>>;

/// Type that represents the &lt;trnData&gt; tag for domain transfer response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// XML schema location for domain response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// The domain name
    pub name: StringValue,
    /// The domain transfer status
    #[serde(rename = "trStatus")]
    pub transfer_status: StringValue,
    /// The epp user who requested the transfer
    #[serde(rename = "reID")]
    pub requester_id: StringValue,
    /// The transfer rquest date
    #[serde(rename = "reDate")]
    pub requested_at: StringValue,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "acID")]
    pub ack_id: StringValue,
    /// THe date by which the acknowledgment should be made
    #[serde(rename = "acDate")]
    pub ack_by: StringValue,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain transfer response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferResult {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferData,
}
