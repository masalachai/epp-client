//! Types for EPP domain check request

use epp_client_macros::*;

use super::EPP_DOMAIN_XMLNS;
use crate::common::{ElementName, EppObject, StringValue};
use crate::request::Command;
use crate::response::CommandResponse;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::check::{EppDomainCheck, EppDomainCheckResponse};
/// use epp_client::generate_client_tr_id;
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
///     // Create an EppDomainCheck instance
///     let domain_check = EppDomainCheck::new(
///         vec!["eppdev-100.com", "eppdev-100.net"],
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainCheckResponse
///     let response = client.transact::<_, EppDomainCheckResponse>(&domain_check).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainCheck = EppObject<Command<DomainCheckRequest>>;

impl EppDomainCheck {
    /// Creates a new EppObject for domain check corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(domains: Vec<&str>, client_tr_id: &str) -> EppDomainCheck {
        let domains = domains.into_iter().map(|d| d.into()).collect();

        let domain_check = DomainCheckRequest {
            list: DomainList {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                domains,
            },
        };

        EppObject::build(Command::<DomainCheckRequest>::new(
            domain_check,
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain check response
pub type EppDomainCheckResponse = EppObject<CommandResponse<DomainCheckResponse>>;

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
