//! Types for EPP message ack request

use epp_client_macros::*;

use crate::common::{ElementName, NoExtension};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MessageAck<E> {
    request: MessageAckRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for MessageAck<E> {
    type Input = MessageAckRequest;
    type Output = String;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> MessageAck<E> {
    pub fn new(message_id: u32) -> MessageAck<NoExtension> {
        MessageAck {
            request: MessageAckRequest {
                op: "ack".to_string(),
                message_id: message_id.to_string(),
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> MessageAck<F> {
        MessageAck {
            request: self.request,
            extension: Some(extension),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAckRequest {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}
