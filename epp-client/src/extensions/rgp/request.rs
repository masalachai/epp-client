//! Types for EPP RGP restore request

use crate::{
    domain::{info::DomainInfo, update::DomainUpdate},
    request::{Extension, Transaction},
};

use serde::{Deserialize, Serialize};

use super::{Update, XMLNS};

impl Transaction<Update<RgpRestoreRequest>> for DomainUpdate {}

impl Transaction<Update<RgpRestoreRequest>> for DomainInfo {}

impl Extension for Update<RgpRestoreRequest> {
    type Response = Update<RgpRequestResponse>;
}

// Request

/// Type corresponding to the &lt;restore&gt; tag for an rgp restore request
#[derive(Serialize, Debug)]
pub struct RgpRestoreRequestData {
    /// The value of the op attribute in the &lt;restore&gt; tag
    pub op: String,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreRequest {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreRequestData,
}

impl Default for RgpRestoreRequest {
    fn default() -> Self {
        Self {
            xmlns: XMLNS.to_string(),
            restore: RgpRestoreRequestData {
                op: "request".to_string(),
            },
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
    use crate::domain::update::DomainUpdate;
    use crate::request::Transaction;
    use crate::tests::{get_xml, SUCCESS_MSG, SVTRID};

    #[test]
    fn rgp_restore_request() {
        let xml = get_xml("response/extensions/rgp_restore.xml").unwrap();
        let object =
            <DomainUpdate as Transaction<Update<RgpRestoreRequest>>>::deserialize_response(
                xml.as_str(),
            )
            .unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(ext.data.rgp_status[0].status, "pendingRestore".to_string());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn rgp_restore_domain_info_response() {
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
