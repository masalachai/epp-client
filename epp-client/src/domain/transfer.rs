//! Types for EPP domain transfer request

use super::XMLNS;
use crate::common::{DomainAuthInfo, NoExtension, Period, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainTransfer {}

impl Command for DomainTransfer {
    type Response = DomainTransferResponse;
    const COMMAND: &'static str = "transfer";
}

impl DomainTransfer {
    pub fn new(name: &str, years: Option<u16>, auth_password: &str) -> Self {
        Self {
            operation: "request".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: years.map(Period::new),
                auth_info: Some(DomainAuthInfo::new(auth_password)),
            },
        }
    }

    pub fn query(name: &str, auth_password: &str) -> Self {
        Self {
            operation: "query".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: None,
                auth_info: Some(DomainAuthInfo::new(auth_password)),
            },
        }
    }

    pub fn approve(name: &str) -> Self {
        Self {
            operation: "approve".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: None,
                auth_info: None,
            },
        }
    }

    pub fn reject(name: &str) -> Self {
        Self {
            operation: "reject".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: None,
                auth_info: None,
            },
        }
    }

    pub fn cancel(name: &str) -> Self {
        Self {
            operation: "cancel".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: None,
                auth_info: None,
            },
        }
    }
}

// Request

/// Type for elements under the domain &lt;transfer&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainTransferReqData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The name of the domain under transfer
    #[serde(rename = "domain:name")]
    name: StringValue,
    /// The period of renewal upon a successful transfer
    /// Only applicable in case of a transfer request
    #[serde(rename = "domain:period")]
    period: Option<Period>,
    /// The authInfo for the domain under transfer
    /// Only applicable to domain transfer and domain transfer query requests
    #[serde(rename = "domain:authInfo")]
    auth_info: Option<DomainAuthInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;transfer&gt; command for domains
pub struct DomainTransfer {
    /// The transfer operation to perform indicated by the 'op' attr
    /// The values are one of transfer or query
    #[serde(rename = "op")]
    operation: String,
    /// The data under the &lt;transfer&gt; tag in the transfer request
    #[serde(rename = "domain:transfer")]
    domain: DomainTransferReqData,
}

// Response

/// Type that represents the &lt;trnData&gt; tag for domain transfer response
#[derive(Deserialize, Debug)]
pub struct DomainTransferResponseData {
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
    pub expiring_at: Option<StringValue>,
}

/// Type that represents the &lt;resData&gt; tag for domain transfer response
#[derive(Deserialize, Debug)]
pub struct DomainTransferResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferResponseData,
}
