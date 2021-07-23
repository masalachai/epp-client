use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use serde::{Deserialize, Serialize};

pub type EppMessagePoll = EppObject<Command<MessagePoll>>;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
pub struct MessagePoll {
    op: String,
}

impl EppMessagePoll {
    pub fn new(client_tr_id: &str) -> EppMessagePoll {
        EppObject::build(Command::<MessagePoll> {
            command: MessagePoll {
                op: "req".to_string(),
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
