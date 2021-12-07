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

#[cfg(test)]
mod tests {
    use super::MessagePoll;
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
        assert_eq!(result.message_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.message_data.transfer_status, "pending".into());
        assert_eq!(result.message_data.requester_id, "eppdev".into());
        assert_eq!(
            result.message_data.requested_at,
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(result.message_data.ack_id, "ClientY".into());
        assert_eq!(result.message_data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            result.message_data.expiring_at,
            "2022-07-02T14:53:19.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
