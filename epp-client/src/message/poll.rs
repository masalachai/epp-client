use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for MessagePoll {}

impl Command for MessagePoll {
    type Response = MessagePollResponse;
    const COMMAND: &'static str = "poll";
}

// Request

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;poll&gt; command for message poll
pub struct MessagePoll {
    /// The type of operation to perform
    /// The value is "req" for message polling
    op: String,
}

impl Default for MessagePoll {
    fn default() -> Self {
        Self {
            op: "req".to_owned(),
        }
    }
}

// Response

/// Type that represents the &lt;trnData&gt; tag for message poll response
#[derive(Deserialize, Debug)]
pub struct MessageDomainTransferData {
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
#[derive(Deserialize, Debug)]
pub struct MessagePollResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "domain:trnData", alias = "trnData")]
    pub message_data: MessageDomainTransferData,
}
