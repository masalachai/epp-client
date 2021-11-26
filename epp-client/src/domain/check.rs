//! Types for EPP domain check request

use epp_client_macros::*;

use super::EPP_DOMAIN_XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainCheck<E> {
    request: DomainCheckRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainCheck<E> {
    type Input = DomainCheckRequest;
    type Output = DomainCheckResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::check::DomainCheck;
/// use epp_client::generate_client_tr_id;
/// use epp_client::common::NoExtension;
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
///     // Create an DomainCheck instance
///     let domain_check = DomainCheck::<NoExtension>::new(
///         vec!["eppdev-100.com", "eppdev-100.net"],
///     );
///
///     // send it to the registry and receive a response of type EppDomainCheckResponse
///     let response = client.transact_new(domain_check, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainCheck<E> {
    pub fn new(domains: Vec<&str>) -> DomainCheck<NoExtension> {
        DomainCheck {
            request: DomainCheckRequest {
                list: DomainList {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domains: domains
                        .into_iter()
                        .map(|d| d.into())
                        .collect::<Vec<StringValue>>(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainCheck<F> {
        DomainCheck {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    /// XML namespace for domain commands
    pub xmlns: String,
    #[serde(rename = "domain:name", alias = "name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheckRequest {
    /// The object holding the list of domains to be checked
    #[serde(rename = "domain:check", alias = "check")]
    list: DomainList,
}

// Response

/// Type that represents the &lt;name&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAvailable {
    /// The domain name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The domain (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponseDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub domain: DomainAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckResponseData,
}
