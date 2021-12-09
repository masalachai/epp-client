//! Types for EPP message ack request

use crate::common::NoExtension;
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for MessageAck {}

impl Command for MessageAck {
    type Response = String;
    const COMMAND: &'static str = "poll";
}

#[derive(Serialize, Deserialize, Debug)]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAck {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}

impl MessageAck {
    pub fn new(message_id: u32) -> Self {
        Self {
            op: "ack".to_string(),
            message_id: message_id.to_string(),
        }
    }
}
