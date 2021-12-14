//! Types for EPP message ack request

use crate::common::NoExtension;
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for MessageAck<'a> {}

impl<'a> Command for MessageAck<'a> {
    type Response = String;
    const COMMAND: &'static str = "poll";
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAck<'a> {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: &'a str,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}

impl<'a> MessageAck<'a> {
    pub fn new(message_id: u32) -> Self {
        Self {
            op: "ack",
            message_id: message_id.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MessageAck;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/message/ack.xml").unwrap();

        let object = MessageAck::new(12345);

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/message/ack.xml").unwrap();
        let object = MessageAck::deserialize_response(xml.as_str()).unwrap();

        let msg = object.message_queue().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(msg.count, 4);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
