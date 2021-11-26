//! Types for EPP RGP restore report

use epp_client_macros::*;

use crate::common::{ElementName, EppObject, Extension, StringValue};
use crate::domain::update::{DomainChangeInfo, DomainUpdateRequest, DomainUpdateRequestData};
use crate::epp::xml::{EPP_DOMAIN_RGP_EXT_XMLNS, EPP_DOMAIN_XMLNS};
use crate::request::CommandWithExtension;
use crate::response::EppCommandResponse;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain rgp restore report command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::rgp::report::{EppDomainRgpRestoreReport, EppDomainRgpRestoreReportResponse};
/// use epp_client::epp::generate_client_tr_id;
/// use chrono::{DateTime, NaiveDate};
/// use std::str::FromStr;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
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
///     // Create an EppDomainRgpRestoreReport instance
///     let domain_restore_report = EppDomainRgpRestoreReport::new(
///         "eppdev.com",
///         pre_data,
///         post_data,
///         deleted_at,
///         restored_at,
///         restore_reason,
///         &statements,
///         other,
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainRgpRestoreReportResponse
///     let response = client.transact::<_, EppDomainRgpRestoreReportResponse>(&domain_restore_report).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainRgpRestoreReport =
    EppObject<CommandWithExtension<DomainUpdateRequest, RgpRestoreReportRequest>>;

impl EppDomainRgpRestoreReport {
    /// Creates a new EppObject for domain rgp restore report corresponding to the &lt;epp&gt; tag in EPP XML
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        pre_data: &str,
        post_data: &str,
        deleted_at: DateTime<Utc>,
        restored_at: DateTime<Utc>,
        restore_reason: &str,
        statements: &[&str],
        other: &str,
        client_tr_id: &str,
    ) -> EppDomainRgpRestoreReport {
        let statements = statements.iter().map(|&s| s.into()).collect();

        let command = CommandWithExtension::<DomainUpdateRequest, RgpRestoreReportRequest> {
            command: DomainUpdateRequest {
                domain: DomainUpdateRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    add: None,
                    remove: None,
                    change_info: Some(DomainChangeInfo {
                        registrant: None,
                        auth_info: None,
                    }),
                },
            },
            extension: Some(Extension {
                data: RgpRestoreReportRequest {
                    xmlns: EPP_DOMAIN_RGP_EXT_XMLNS.to_string(),
                    restore: RgpRestoreReportRequestSection {
                        op: "report".to_string(),
                        report: RgpRestoreReportRequestData {
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
                },
            }),
            client_tr_id: client_tr_id.into(),
        };

        EppObject::build(command)
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML rgp restore report response
pub type EppDomainRgpRestoreReportResponse = EppCommandResponse;

/// Type corresponding to the &lt;report&gt; section in the EPP rgp restore extension
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpRestoreReportRequestData {
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
pub struct RgpRestoreReportRequestSection {
    /// The value of the op attribute for the &lt;restore&gt; tag
    op: String,
    /// Data for the &lt;report&gt; tag
    #[serde(rename = "rgp:report", alias = "report")]
    report: RgpRestoreReportRequestData,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "rgp:update")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreReportRequest {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreReportRequestSection,
}
