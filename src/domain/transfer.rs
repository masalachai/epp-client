//! Types for EPP domain transfer request

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{DomainAuthInfo, Period, XMLNS};
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for DomainTransfer<'a> {}

impl<'a> Command for DomainTransfer<'a> {
    type Response = DomainTransferResponse;
    const COMMAND: &'static str = "transfer";
}

impl<'a> DomainTransfer<'a> {
    pub fn new(name: &'a str, period: Option<Period>, auth_password: &'a str) -> Self {
        Self::build(
            "request",
            name,
            period,
            Some(DomainAuthInfo::new(auth_password)),
        )
    }

    pub fn query(name: &'a str, auth_password: &'a str) -> Self {
        Self::build(
            "query",
            name,
            None,
            Some(DomainAuthInfo::new(auth_password)),
        )
    }

    pub fn approve(name: &'a str) -> Self {
        Self::build("approve", name, None, None)
    }

    pub fn reject(name: &'a str) -> Self {
        Self::build("reject", name, None, None)
    }

    pub fn cancel(name: &'a str) -> Self {
        Self::build("cancel", name, None, None)
    }

    fn build(
        operation: &'a str,
        name: &'a str,
        period: Option<Period>,
        auth_info: Option<DomainAuthInfo<'a>>,
    ) -> Self {
        Self {
            operation,
            domain: DomainTransferReqData {
                xmlns: XMLNS,
                name: name.into(),
                period,
                auth_info,
            },
        }
    }
}

// Request

/// Type for elements under the domain &lt;transfer&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainTransferReqData<'a> {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    xmlns: &'a str,
    /// The name of the domain under transfer
    #[serde(rename = "domain:name")]
    name: StringValue<'a>,
    /// The period of renewal upon a successful transfer
    /// Only applicable in case of a transfer request
    #[serde(rename = "domain:period")]
    period: Option<Period>,
    /// The authInfo for the domain under transfer
    /// Only applicable to domain transfer and domain transfer query requests
    #[serde(rename = "domain:authInfo")]
    auth_info: Option<DomainAuthInfo<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;transfer&gt; command for domains
pub struct DomainTransfer<'a> {
    /// The transfer operation to perform indicated by the 'op' attr
    /// The values are one of transfer or query
    #[serde(rename = "op")]
    operation: &'a str,
    /// The data under the &lt;transfer&gt; tag in the transfer request
    #[serde(rename = "domain:transfer")]
    domain: DomainTransferReqData<'a>,
}

// Response

/// Type that represents the &lt;trnData&gt; tag for domain transfer response
#[derive(Deserialize, Debug)]
pub struct DomainTransferResponseData {
    /// The domain name
    pub name: StringValue<'static>,
    /// The domain transfer status
    #[serde(rename = "trStatus")]
    pub transfer_status: StringValue<'static>,
    /// The epp user who requested the transfer
    #[serde(rename = "reID")]
    pub requester_id: StringValue<'static>,
    /// The transfer rquest date
    #[serde(rename = "reDate")]
    pub requested_at: DateTime<Utc>,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "acID")]
    pub ack_id: StringValue<'static>,
    /// THe date by which the acknowledgment should be made
    #[serde(rename = "acDate")]
    pub ack_by: DateTime<Utc>,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: Option<DateTime<Utc>>,
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
    use chrono::{TimeZone, Utc};

    use super::{DomainTransfer, Period};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn request_command() {
        let object =
            DomainTransfer::new("testing.com", Some(Period::years(1).unwrap()), "epP4uthd#v");
        assert_serialized("request/domain/transfer_request.xml", &object);
    }

    #[test]
    fn approve_command() {
        let object = DomainTransfer::approve("testing.com");
        assert_serialized("request/domain/transfer_approve.xml", &object);
    }

    #[test]
    fn reject_command() {
        let object = DomainTransfer::reject("testing.com");
        assert_serialized("request/domain/transfer_reject.xml", &object);
    }

    #[test]
    fn cancel_command() {
        let object = DomainTransfer::cancel("testing.com");
        assert_serialized("request/domain/transfer_cancel.xml", &object);
    }

    #[test]
    fn query_command() {
        let object = DomainTransfer::query("testing.com", "epP4uthd#v");
        assert_serialized("request/domain/transfer_query.xml", &object);
    }

    #[test]
    fn request_response() {
        let object = response_from_file::<DomainTransfer>("response/domain/transfer_request.xml");

        let result = object.res_data().unwrap();

        assert_eq!(
            object.result.code,
            ResultCode::CommandCompletedSuccessfullyActionPending
        );
        assert_eq!(
            object.result.message,
            "Command completed successfully; action pending".into()
        );
        assert_eq!(result.transfer_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.transfer_data.transfer_status, "pending".into());
        assert_eq!(result.transfer_data.requester_id, "eppdev".into());
        assert_eq!(
            result.transfer_data.requested_at,
            Utc.with_ymd_and_hms(2021, 7, 23, 15, 31, 21).unwrap(),
        );
        assert_eq!(result.transfer_data.ack_id, "ClientY".into());
        assert_eq!(
            result.transfer_data.ack_by,
            Utc.with_ymd_and_hms(2021, 7, 28, 15, 31, 21).unwrap()
        );
        assert_eq!(
            result.transfer_data.expiring_at,
            Utc.with_ymd_and_hms(2022, 7, 2, 14, 53, 19).single(),
        );
        assert_eq!(*object.tr_ids.client_tr_id.as_ref().unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn approve_response() {
        let object = response_from_file::<DomainTransfer>("response/domain/transfer_approve.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn reject_response() {
        let object = response_from_file::<DomainTransfer>("response/domain/transfer_reject.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn cancel_response() {
        let object = response_from_file::<DomainTransfer>("response/domain/transfer_cancel.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn query_response() {
        let object = response_from_file::<DomainTransfer>("response/domain/transfer_query.xml");

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.transfer_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.transfer_data.transfer_status, "pending".into());
        assert_eq!(result.transfer_data.requester_id, "eppdev".into());
        assert_eq!(
            result.transfer_data.requested_at,
            Utc.with_ymd_and_hms(2021, 7, 23, 15, 31, 21).unwrap()
        );
        assert_eq!(result.transfer_data.ack_id, "ClientY".into());
        assert_eq!(
            result.transfer_data.ack_by,
            Utc.with_ymd_and_hms(2021, 7, 28, 15, 31, 21).unwrap()
        );
        assert_eq!(
            result.transfer_data.expiring_at,
            Utc.with_ymd_and_hms(2022, 7, 2, 14, 53, 19).single()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
