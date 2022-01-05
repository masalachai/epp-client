use crate::common::NoExtension;
use crate::domain::transfer::DomainTransferResponseData;
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl<'a> Transaction<NoExtension> for MessagePoll<'a> {}

impl<'a> Command for MessagePoll<'a> {
    type Response = MessagePollResponse;
    const COMMAND: &'static str = "poll";
}

// Request

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;poll&gt; command for message poll
pub struct MessagePoll<'a> {
    /// The type of operation to perform
    /// The value is "req" for message polling
    op: &'a str,
}

impl Default for MessagePoll<'static> {
    fn default() -> Self {
        Self { op: "req" }
    }
}

// Response

/// Type that represents the &lt;trnData&gt; tag for message poll response
#[derive(Deserialize, Debug)]
pub enum MessageData {
    /// Data under the &lt;domain:trnData&gt; tag
    #[serde(rename = "domain:trnData")]
    DomainTransfer(DomainTransferResponseData),
}

/// Type that represents the &lt;resData&gt; tag for message poll response
#[derive(Deserialize, Debug)]
pub struct MessagePollResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(alias = "trnData")]
    pub message_data: MessageData,
}

#[cfg(test)]
mod tests {
    use super::MessagePoll;
    use crate::message::poll::MessageData;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/message/poll.xml").unwrap();

        let object = MessagePoll::default();

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/message/poll.xml").unwrap();
        let object = MessagePoll::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();
        let msg = object.message_queue().unwrap();

        assert_eq!(object.result.code, 1301);
        assert_eq!(
            object.result.message,
            "Command completed successfully; ack to dequeue".into()
        );
        assert_eq!(msg.count, 5);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(
            *(msg.date.as_ref().unwrap()),
            "2021-07-23T19:12:43.0Z".into()
        );
        assert_eq!(
            *(msg.message.as_ref().unwrap()),
            "Transfer requested.".into()
        );

        let MessageData::DomainTransfer(data) = &result.message_data;

        assert_eq!(data.name, "eppdev-transfer.com".into());
        assert_eq!(data.transfer_status, "pending".into());
        assert_eq!(data.requester_id, "eppdev".into());
        assert_eq!(data.requested_at, "2021-07-23T15:31:21.0Z".into());
        assert_eq!(data.ack_id, "ClientY".into());
        assert_eq!(data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            *data.expiring_at.as_ref().unwrap(),
            "2022-07-02T14:53:19.0Z".into()
        );

        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
