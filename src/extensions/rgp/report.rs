//! Types for EPP RGP restore report

use chrono::{DateTime, SecondsFormat, Utc};
use instant_xml::ToXml;

use crate::common::NoExtension;
use crate::domain::update::DomainUpdate;
use crate::request::{Extension, Transaction};

use super::XMLNS;

impl<'a> Transaction<Update<RgpRestoreReport<'a>>> for DomainUpdate<'a> {}

impl<'a> RgpRestoreReport<'a> {
    /// Create a new RGP restore report request
    pub fn new(
        pre_data: &'a str,
        post_data: &'a str,
        deleted_at: DateTime<Utc>,
        restored_at: DateTime<Utc>,
        restore_reason: &'a str,
        statements: &'a [&'a str],
        other: &'a str,
    ) -> Self {
        Self {
            op: "report",
            report: RgpRestoreReportSectionData {
                pre_data,
                post_data,
                deleted_at: deleted_at.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                restored_at: restored_at.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                restore_reason,
                statements,
                other,
            },
        }
    }
}

impl<'a> Extension for Update<RgpRestoreReport<'a>> {
    type Response = NoExtension;
}

#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(XMLNS))]
pub struct Update<T> {
    pub data: T,
}

/// Type corresponding to the &lt;report&gt; section in the EPP rgp restore extension
#[derive(Debug, ToXml)]
#[xml(rename = "report", ns(XMLNS))]
pub struct RgpRestoreReportSectionData<'a> {
    /// The pre-delete registration date
    #[xml(rename = "preData")]
    pre_data: &'a str,
    /// The post-delete registration date
    #[xml(rename = "postData")]
    post_data: &'a str,
    /// The domain deletion date
    #[xml(rename = "delTime")]
    deleted_at: String,
    /// The domain restore request date
    #[xml(rename = "resTime")]
    restored_at: String,
    /// The reason for domain restoration
    #[xml(rename = "resReason")]
    restore_reason: &'a str,
    /// The registrar's statements on the domain restoration
    #[xml(rename = "statement")]
    statements: &'a [&'a str],
    /// Other remarks for domain restoration
    #[xml(rename = "other")]
    other: &'a str,
}

#[derive(Debug, ToXml)]
/// Type for EPP XML &lt;check&gt; command for domains
#[xml(rename = "restore", ns(XMLNS))]
pub struct RgpRestoreReport<'a> {
    /// The value of the op attribute for the &lt;restore&gt; tag
    #[xml(attribute)]
    op: &'a str,
    /// Data for the &lt;report&gt; tag
    #[xml(rename = "rgp:report")]
    report: RgpRestoreReportSectionData<'a>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::DateTime;

    use super::{RgpRestoreReport, Update};
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::tests::assert_serialized;

    #[test]
    fn command() {
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

        assert_serialized(
            "request/extensions/rgp_restore_report.xml",
            (&object, &domain_restore_report),
        );
    }
}
