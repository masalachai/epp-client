//! Types for EPP message poll request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use crate::epp::request::Command;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for registry <poll op="req"> command
pub type EppMessagePoll = EppObject<Command<MessagePoll>>;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML <poll> command for message poll
pub struct MessagePoll {
    /// The type of operation to perform
    /// The value is "req" for message polling
    op: String,
}

impl EppMessagePoll {
    /// Creates a new EppObject for <poll> req corresponding to the <epp> tag in EPP XML
    pub fn new(client_tr_id: &str) -> EppMessagePoll {
        EppObject::build(Command::<MessagePoll> {
            command: MessagePoll {
                op: "req".to_string(),
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
