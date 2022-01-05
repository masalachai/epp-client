use crate::common::NoExtension;
use crate::domain::transfer::DomainTransferResponseData;
use crate::host::info::HostInfoResponseData;
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
    /// Data under the &lt;host:infData&gt; tag
    #[serde(rename = "host:infData")]
    HostInfo(HostInfoResponseData),
}

/// Type that represents the &lt;resData&gt; tag for message poll response
#[derive(Deserialize, Debug)]
pub struct MessagePollResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(alias = "trnData", alias = "infData")]
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
    fn domain_transfer_response() {
        let xml = get_xml("response/message/poll_domain_transfer.xml").unwrap();
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

        if let MessageData::DomainTransfer(tr) = &result.message_data {
            assert_eq!(tr.name, "eppdev-transfer.com".into());
            assert_eq!(tr.transfer_status, "pending".into());
            assert_eq!(tr.requester_id, "eppdev".into());
            assert_eq!(tr.requested_at, "2021-07-23T15:31:21.0Z".into());
            assert_eq!(tr.ack_id, "ClientY".into());
            assert_eq!(tr.ack_by, "2021-07-28T15:31:21.0Z".into());
            assert_eq!(
                *tr.expiring_at.as_ref().unwrap(),
                "2022-07-02T14:53:19.0Z".into()
            );
        } else {
            panic!("Wrong type");
        }

        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn host_info_response() {
        let xml = get_xml("response/message/poll_host_info.xml").unwrap();
        let object = MessagePoll::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();
        let msg = object.message_queue().unwrap();

        assert_eq!(object.result.code, 1301);
        assert_eq!(
            object.result.message,
            "Command completed successfully; ack to dequeue".into()
        );
        assert_eq!(msg.count, 4);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(*(msg.date.as_ref().unwrap()), "2022-01-02T11:30:45Z".into());
        assert_eq!(
            *(msg.message.as_ref().unwrap()),
            "Unused objects policy".into()
        );

        if let MessageData::HostInfo(host) = &result.message_data {
            assert_eq!(host.name, "ns.test.com".into());

            assert_eq!(host.roid, "1234".into());
            assert!(host.statuses.iter().any(|s| s.status == "ok"));
            assert!(host.addresses.iter().any(|a| a.address == *"1.1.1.1"));
            assert_eq!(host.client_id, "1234".into());
            assert_eq!(host.creator_id, "user".into());
            assert_eq!(host.created_at, "2021-12-01T22:40:48Z".into());
            assert_eq!(host.updater_id, Some("user".into()));
            assert_eq!(host.updated_at, Some("2021-12-01T22:40:48Z".into()));
        } else {
            panic!("Wrong type");
        }

        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn message_only_response() {
        let xml = get_xml("response/message/poll_message_only.xml").unwrap();
        let object = MessagePoll::deserialize_response(xml.as_str()).unwrap();

        let msg = object.message_queue().unwrap();

        assert_eq!(object.result.code, 1301);
        assert_eq!(
            object.result.message,
            "Command completed successfully; ack to dequeue".into()
        );

        assert_eq!(msg.count, 4);
        assert_eq!(msg.id, "12346".to_string());
        assert_eq!(
            *(msg.date.as_ref().unwrap()),
            "2000-06-08T22:10:00.0Z".into()
        );
        assert_eq!(
            *(msg.message.as_ref().unwrap()),
            "Credit balance low.".into()
        );

        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn empty_queue_response() {
        let xml = get_xml("response/message/poll_empty_queue.xml").unwrap();
        let object = MessagePoll::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1300);
        assert_eq!(
            object.result.message,
            "Command completed successfully; no messages".into()
        );

        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
