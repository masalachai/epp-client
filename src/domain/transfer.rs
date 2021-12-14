//! Types for EPP domain transfer request

use super::{DomainAuthInfo, Period, XMLNS};
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainTransfer {}

impl Command for DomainTransfer {
    type Response = DomainTransferResponse;
    const COMMAND: &'static str = "transfer";
}

impl DomainTransfer {
    pub fn new(name: &str, years: Option<u16>, auth_password: &str) -> Self {
        Self {
            operation: "request".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS,
                name: name.into(),
                period: years.map(Period::new),
                auth_info: Some(DomainAuthInfo::new(auth_password)),
            },
        }
    }

    pub fn query(name: &str, auth_password: &str) -> Self {
        Self {
            operation: "query".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS,
                name: name.into(),
                period: None,
                auth_info: Some(DomainAuthInfo::new(auth_password)),
            },
        }
    }

    pub fn approve(name: &str) -> Self {
        Self {
            operation: "approve".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS,
                name: name.into(),
                period: None,
                auth_info: None,
            },
        }
    }

    pub fn reject(name: &str) -> Self {
        Self {
            operation: "reject".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS,
                name: name.into(),
                period: None,
                auth_info: None,
            },
        }
    }

    pub fn cancel(name: &str) -> Self {
        Self {
            operation: "cancel".to_string(),
            domain: DomainTransferReqData {
                xmlns: XMLNS,
                name: name.into(),
                period: None,
                auth_info: None,
            },
        }
    }
}

// Request

/// Type for elements under the domain &lt;transfer&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainTransferReqData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    xmlns: &'static str,
    /// The name of the domain under transfer
    #[serde(rename = "domain:name")]
    name: StringValue,
    /// The period of renewal upon a successful transfer
    /// Only applicable in case of a transfer request
    #[serde(rename = "domain:period")]
    period: Option<Period>,
    /// The authInfo for the domain under transfer
    /// Only applicable to domain transfer and domain transfer query requests
    #[serde(rename = "domain:authInfo")]
    auth_info: Option<DomainAuthInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;transfer&gt; command for domains
pub struct DomainTransfer {
    /// The transfer operation to perform indicated by the 'op' attr
    /// The values are one of transfer or query
    #[serde(rename = "op")]
    operation: String,
    /// The data under the &lt;transfer&gt; tag in the transfer request
    #[serde(rename = "domain:transfer")]
    domain: DomainTransferReqData,
}

// Response

/// Type that represents the &lt;trnData&gt; tag for domain transfer response
#[derive(Deserialize, Debug)]
pub struct DomainTransferResponseData {
    /// The domain name
    pub name: StringValue,
    /// The domain transfer status
    #[serde(rename = "trStatus")]
    pub transfer_status: StringValue,
    /// The epp user who requested the transfer
    #[serde(rename = "reID")]
    pub requester_id: StringValue,
    /// The transfer rquest date
    #[serde(rename = "reDate")]
    pub requested_at: StringValue,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "acID")]
    pub ack_id: StringValue,
    /// THe date by which the acknowledgment should be made
    #[serde(rename = "acDate")]
    pub ack_by: StringValue,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: Option<StringValue>,
}

/// Type that represents the &lt;resData&gt; tag for domain transfer response
#[derive(Deserialize, Debug)]
pub struct DomainTransferResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferResponseData,
}

#[cfg(test)]
mod tests {
    use super::DomainTransfer;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn request_command() {
        let xml = get_xml("request/domain/transfer_request.xml").unwrap();

        let object = DomainTransfer::new("testing.com", Some(1), "epP4uthd#v");

        let serialized =
            <DomainTransfer as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn approve_command() {
        let xml = get_xml("request/domain/transfer_approve.xml").unwrap();

        let object = DomainTransfer::approve("testing.com");

        let serialized =
            <DomainTransfer as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn reject_command() {
        let xml = get_xml("request/domain/transfer_reject.xml").unwrap();

        let object = DomainTransfer::reject("testing.com");

        let serialized =
            <DomainTransfer as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn cancel_command() {
        let xml = get_xml("request/domain/transfer_cancel.xml").unwrap();

        let object = DomainTransfer::cancel("testing.com");

        let serialized =
            <DomainTransfer as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn query_command() {
        let xml = get_xml("request/domain/transfer_query.xml").unwrap();

        let object = DomainTransfer::query("testing.com", "epP4uthd#v");

        let serialized =
            <DomainTransfer as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn request_response() {
        let xml = get_xml("response/domain/transfer_request.xml").unwrap();
        let object =
            <DomainTransfer as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1001);
        assert_eq!(
            object.result.message,
            "Command completed successfully; action pending".into()
        );
        assert_eq!(result.transfer_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.transfer_data.transfer_status, "pending".into());
        assert_eq!(result.transfer_data.requester_id, "eppdev".into());
        assert_eq!(
            result.transfer_data.requested_at,
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(result.transfer_data.ack_id, "ClientY".into());
        assert_eq!(result.transfer_data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            *result.transfer_data.expiring_at.as_ref().unwrap(),
            "2022-07-02T14:53:19.0Z".into()
        );
        assert_eq!(*object.tr_ids.client_tr_id.as_ref().unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn approve_response() {
        let xml = get_xml("response/domain/transfer_approve.xml").unwrap();
        let object =
            <DomainTransfer as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn reject_response() {
        let xml = get_xml("response/domain/transfer_reject.xml").unwrap();
        let object =
            <DomainTransfer as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn cancel_response() {
        let xml = get_xml("response/domain/transfer_cancel.xml").unwrap();
        let object =
            <DomainTransfer as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn query_response() {
        let xml = get_xml("response/domain/transfer_query.xml").unwrap();
        let object =
            <DomainTransfer as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.transfer_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.transfer_data.transfer_status, "pending".into());
        assert_eq!(result.transfer_data.requester_id, "eppdev".into());
        assert_eq!(
            result.transfer_data.requested_at,
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(result.transfer_data.ack_id, "ClientY".into());
        assert_eq!(result.transfer_data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            *result.transfer_data.expiring_at.as_ref().unwrap(),
            "2022-07-02T14:53:19.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
