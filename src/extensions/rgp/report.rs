//! Types for EPP RGP restore report

use crate::common::{NoExtension, StringValue};
use crate::domain::update::DomainUpdate;
use crate::request::{Extension, Transaction};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;

use super::{Update, XMLNS};

impl Transaction<Update<RgpRestoreReport>> for DomainUpdate {}

impl RgpRestoreReport {
    /// Create a new RGP restore report request
    pub fn new(
        pre_data: &str,
        post_data: &str,
        deleted_at: DateTime<Utc>,
        restored_at: DateTime<Utc>,
        restore_reason: &str,
        statements: &[&str],
        other: &str,
    ) -> RgpRestoreReport {
        let statements = statements.iter().map(|&s| s.into()).collect();

        RgpRestoreReport {
            xmlns: XMLNS,
            restore: RgpRestoreReportSection {
                op: "report".to_string(),
                report: RgpRestoreReportSectionData {
                    pre_data: pre_data.into(),
                    post_data: post_data.into(),
                    deleted_at: deleted_at
                        .to_rfc3339_opts(SecondsFormat::AutoSi, true)
                        .into(),
                    restored_at: restored_at
                        .to_rfc3339_opts(SecondsFormat::AutoSi, true)
                        .into(),
                    restore_reason: restore_reason.into(),
                    statements,
                    other: other.into(),
                },
            },
        }
    }
}

impl Extension for Update<RgpRestoreReport> {
    type Response = NoExtension;
}

/// Type corresponding to the &lt;report&gt; section in the EPP rgp restore extension
#[derive(Serialize, Debug)]
pub struct RgpRestoreReportSectionData {
    /// The pre-delete registration date
    #[serde(rename = "rgp:preData")]
    pre_data: StringValue,
    /// The post-delete registration date
    #[serde(rename = "rgp:postData")]
    post_data: StringValue,
    /// The domain deletion date
    #[serde(rename = "rgp:delTime")]
    deleted_at: StringValue,
    /// The domain restore request date
    #[serde(rename = "rgp:resTime")]
    restored_at: StringValue,
    /// The reason for domain restoration
    #[serde(rename = "rgp:resReason")]
    restore_reason: StringValue,
    /// The registrar's statements on the domain restoration
    #[serde(rename = "rgp:statement")]
    statements: Vec<StringValue>,
    /// Other remarks for domain restoration
    #[serde(rename = "rgp:other")]
    other: StringValue,
}

/// Type corresponding to the &lt;restore&gt; section in the rgp restore extension
#[derive(Serialize, Debug)]
pub struct RgpRestoreReportSection {
    /// The value of the op attribute for the &lt;restore&gt; tag
    op: String,
    /// Data for the &lt;report&gt; tag
    #[serde(rename = "rgp:report")]
    report: RgpRestoreReportSectionData,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreReport {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp")]
    xmlns: &'static str,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore")]
    restore: RgpRestoreReportSection,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::DateTime;

    use super::{RgpRestoreReport, Update};
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/extensions/rgp_restore_report.xml").unwrap();

        let pre_data =
            "Pre-delete registration data goes here. Both XML and free text are allowed.";
        let post_data =
            "Post-restore registration data goes here. Both XML and free text are allowed.";
        let deleted_at = DateTime::from_str("2021-07-10T22:00:00.0Z").unwrap();
        let restored_at = DateTime::from_str("2021-07-20T22:00:00.0Z").unwrap();
        let restore_reason = "Registrant error.";
        let statements = &[
        "This registrar has not restored the Registered Name in order to assume the rights to use or sell the Registered Name for itself or for any third party.",
        "The information in this report is true to best of this registrar's knowledge, and this registrar acknowledges that intentionally supplying false information in this report shall constitute an incurable material breach of the Registry-Registrar Agreement.",
    ];
        let other = "Supporting information goes here.";

        let domain_restore_report = Update {
            data: RgpRestoreReport::new(
                pre_data,
                post_data,
                deleted_at,
                restored_at,
                restore_reason,
                statements,
                other,
            ),
        };

        let mut object = DomainUpdate::new("eppdev.com");
        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        let serialized =
            <DomainUpdate as Transaction<Update<RgpRestoreReport>>>::serialize_request(
                &object,
                Some(&domain_restore_report),
                CLTRID,
            )
            .unwrap();

        assert_eq!(xml, serialized);
    }
}
