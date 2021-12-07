//! Types for EPP message poll request

use epp_client_macros::*;

use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MessagePoll<E> {
    request: MessagePollRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for MessagePoll<E> {
    type Input = MessagePollRequest;
    type Output = MessagePollResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> MessagePoll<E> {
    pub fn new() -> MessagePoll<NoExtension> {
        MessagePoll {
            request: MessagePollRequest {
                op: "req".to_string(),
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> MessagePoll<F> {
        MessagePoll {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message poll
pub struct MessagePollRequest {
    /// The type of operation to perform
    /// The value is "req" for message polling
    op: String,
}

// Response

/// Type that represents the &lt;trnData&gt; tag for message poll response
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDomainTransferData {
    /// XML namespace for message response data
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The name of the domain under transfer
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// The domain transfer status
    #[serde(rename = "domain:trStatus", alias = "trStatus")]
    pub transfer_status: StringValue,
    /// The epp user who requested the transfer
    #[serde(rename = "domain:reID", alias = "reID")]
    pub requester_id: StringValue,
    /// The date of the transfer request
    #[serde(rename = "domain:reDate", alias = "reDate")]
    pub requested_at: StringValue,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "domain:acID", alias = "acID")]
    pub ack_id: StringValue,
    /// The date by which the transfer request should be acknowledged
    #[serde(rename = "domain:acDate", alias = "acDate")]
    pub ack_by: StringValue,
    /// The domain expiry date
    #[serde(rename = "domain:exDate", alias = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for message poll response
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePollResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "domain:trnData", alias = "trnData")]
    pub message_data: MessageDomainTransferData,
}
