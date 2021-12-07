//! Types for EPP RGP restore report

use epp_client_macros::*;

use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::EppExtension;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

use super::XMLNS;

/// Type that represents the domain rgp restore report extension
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::common::{DomainStatus, DomainContact};
/// use epp_client::extensions::rgp::report::RgpRestoreReport;
/// use epp_client::domain::update::DomainUpdate;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
/// use chrono::{DateTime, NaiveDate};
/// use std::str::FromStr;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     let pre_data =
///         "Pre-delete registration data goes here. Both XML and free text are allowed.";
///     let post_data =
///         "Post-restore registration data goes here. Both XML and free text are allowed.";
///     let deleted_at = DateTime::from_str("2021-07-10T22:00:00.0Z").unwrap();
///     let restored_at = DateTime::from_str("2021-07-20T22:00:00.0Z").unwrap();
///     let restore_reason = "Registrant error.";
///     let statements = vec![
///         "This registrar has not restored the Registered Name in order to assume the rights to use or sell the Registered Name for itself or for any third party.",
///         "The information in this report is true to best of this registrar's knowledge, and this registrar acknowledges that intentionally supplying false information in this report shall constitute an incurable material breach of the Registry-Registrar Agreement.",
///     ];
///     let other = "Supporting information goes here.";
///
///     let domain_restore_report = RgpRestoreReport::new(
///         pre_data,
///         post_data,
///         deleted_at,
///         restored_at,
///         restore_reason,
///         &statements,
///         other
///     );
///
///     // Create an DomainUpdate instance
///     let mut domain_update = DomainUpdate::<RgpRestoreReport>::new("eppdev-100.com").with_extension(domain_restore_report);
///
///     // send it to the registry and receive a response of type EppDomainUpdateResponse
///     let response = client.transact(domain_update, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
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

impl EppExtension for RgpRestoreReport {
    type Response = NoExtension;
}

/// Type corresponding to the &lt;report&gt; section in the EPP rgp restore extension
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpRestoreReportSection {
    /// The value of the op attribute for the &lt;restore&gt; tag
    op: String,
    /// Data for the &lt;report&gt; tag
    #[serde(rename = "rgp:report", alias = "report")]
    report: RgpRestoreReportSectionData,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "rgp:update")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreReport {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreReportSection,
}
