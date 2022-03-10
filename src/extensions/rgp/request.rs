//! Types for EPP RGP restore request

use crate::{
    domain::{info::DomainInfo, update::DomainUpdate},
    request::{Extension, Transaction},
};

use serde::{Deserialize, Serialize};

use super::{Update, XMLNS};

impl<'a> Transaction<Update<RgpRestoreRequest<'a>>> for DomainUpdate<'a> {}

impl<'a> Transaction<Update<RgpRestoreRequest<'a>>> for DomainInfo<'a> {}

impl<'a> Extension for Update<RgpRestoreRequest<'a>> {
    type Response = Update<RgpRequestResponse>;
}

// Request

/// Type corresponding to the &lt;restore&gt; tag for an rgp restore request
#[derive(Serialize, Debug)]
pub struct RgpRestoreRequestData<'a> {
    /// The value of the op attribute in the &lt;restore&gt; tag
    pub op: &'a str,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreRequest<'a> {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp")]
    xmlns: &'a str,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore")]
    restore: RgpRestoreRequestData<'a>,
}

impl Default for RgpRestoreRequest<'static> {
    fn default() -> Self {
        Self {
            xmlns: XMLNS,
            restore: RgpRestoreRequestData { op: "request" },
        }
    }
}

// Response

/// Type that represents the &lt;rgpStatus&gt; tag for domain rgp restore request response
#[derive(Deserialize, Debug)]
pub struct RgpStatus {
    /// The domain RGP status
    #[serde(rename = "s")]
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "upData")]
/// Type that represents the &lt;resData&gt; tag for domain transfer response
pub struct RgpRequestResponse {
    /// Data under the &lt;rgpStatus&gt; tag
    #[serde(rename = "rgpStatus")]
    pub rgp_status: Vec<RgpStatus>,
}

#[cfg(test)]
mod tests {
    use super::{RgpRestoreRequest, Update};
    use crate::domain::info::DomainInfo;
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::request::Transaction;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, get_xml, SUCCESS_MSG, SVTRID};

    #[test]
    fn request_command() {
        let domain_restore_request = Update {
            data: RgpRestoreRequest::default(),
        };

        let mut object = DomainUpdate::new("eppdev.com");

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: None,
        };

        object.info(change_info);

        assert_serialized(
            "request/extensions/rgp_restore_request.xml",
            (&object, &domain_restore_request),
        );
    }

    #[test]
    fn request_response() {
        let xml = get_xml("response/extensions/rgp_restore.xml").unwrap();
        let object =
            <DomainUpdate as Transaction<Update<RgpRestoreRequest>>>::deserialize_response(
                xml.as_str(),
            )
            .unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(ext.data.rgp_status[0].status, "pendingRestore".to_string());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_info_request_response() {
        let xml = get_xml("response/extensions/domain_info_rgp.xml").unwrap();
        let object = <DomainInfo as Transaction<Update<RgpRestoreRequest>>>::deserialize_response(
            xml.as_str(),
        )
        .unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(ext.data.rgp_status[0].status, "addPeriod");
        assert_eq!(ext.data.rgp_status[1].status, "renewPeriod");
    }
}
