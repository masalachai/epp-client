//! Types for EPP message ack request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use crate::epp::request::Command;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for registry <poll op="ack"> command
pub type EppMessageAck = EppObject<Command<MessageAck>>;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML <poll> command for message ack
pub struct MessageAck {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}

impl EppMessageAck {
    /// Creates a new EppObject for <poll> ack corresponding to the <epp> tag in EPP XML
    pub fn new(message_id: u32, client_tr_id: &str) -> EppMessageAck {
        EppObject::build(Command::<MessageAck> {
            command: MessageAck {
                op: "ack".to_string(),
                message_id: message_id.to_string(),
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
