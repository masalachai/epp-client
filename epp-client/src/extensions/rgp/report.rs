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
            xmlns: XMLNS.to_string(),
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
    #[serde(rename = "rgp:preData", alias = "preData")]
    pre_data: StringValue,
    /// The post-delete registration date
    #[serde(rename = "rgp:postData", alias = "postData")]
    post_data: StringValue,
    /// The domain deletion date
    #[serde(rename = "rgp:delTime", alias = "delTime")]
    deleted_at: StringValue,
    /// The domain restore request date
    #[serde(rename = "rgp:resTime", alias = "resTime")]
    restored_at: StringValue,
    /// The reason for domain restoration
    #[serde(rename = "rgp:resReason", alias = "resReason")]
    restore_reason: StringValue,
    /// The registrar's statements on the domain restoration
    #[serde(rename = "rgp:statement", alias = "statement")]
    statements: Vec<StringValue>,
    /// Other remarks for domain restoration
    #[serde(rename = "rgp:other", alias = "other")]
    other: StringValue,
}

/// Type corresponding to the &lt;restore&gt; section in the rgp restore extension
#[derive(Serialize, Debug)]
pub struct RgpRestoreReportSection {
    /// The value of the op attribute for the &lt;restore&gt; tag
    op: String,
    /// Data for the &lt;report&gt; tag
    #[serde(rename = "rgp:report", alias = "report")]
    report: RgpRestoreReportSectionData,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreReport {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreReportSection,
}
