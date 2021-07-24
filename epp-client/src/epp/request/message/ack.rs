use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use serde::{Deserialize, Serialize};

pub type EppMessageAck = EppObject<Command<MessageAck>>;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
pub struct MessageAck {
    op: String,
    #[serde(rename = "msgID")]
    message_id: String,
}

impl EppMessageAck {
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
