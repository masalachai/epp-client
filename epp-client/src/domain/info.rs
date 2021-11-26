//! Types for EPP domain info request

use epp_client_macros::*;

use super::EPP_DOMAIN_XMLNS;
use crate::common::{
    DomainAuthInfo, DomainContact, DomainStatus, ElementName, HostAttr, NoExtension, StringValue,
};
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainInfo<E> {
    request: DomainInfoRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainInfo<E> {
    type Input = DomainInfoRequest;
    type Output = DomainInfoResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for domain &lt;info&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::info::DomainInfo;
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
///     // Create an DomainInfo instance
///     let domain_info = DomainInfo::<NoExtension>::new("eppdev-100.com");
///
///     // send it to the registry and receive a response of type DomainInfoResponse
///     let response = client.transact_new(domain_info, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainInfo<E> {
    pub fn new(name: &str) -> DomainInfo<NoExtension> {
        DomainInfo {
            request: DomainInfoRequest {
                info: DomainInfoRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domain: Domain {
                        hosts: "all".to_string(),
                        name: name.to_string(),
                    },
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainInfo<F> {
        DomainInfo {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for data under the &lt;name&gt; element tag for the domain &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    /// The hosts attribute. Default value is "all"
    hosts: String,
    /// The name of the domain
    #[serde(rename = "$value")]
    name: String,
}

/// Type for &lt;name&gt; element under the domain &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The data for the domain to be queried
    #[serde(rename = "domain:name", alias = "name")]
    domain: Domain,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for domains
pub struct DomainInfoRequest {
    /// The data under the &lt;info&gt; tag for domain info
    #[serde(rename = "domain:info", alias = "info")]
    info: DomainInfoRequestData,
}

// Response

/// The two types of ns lists, hostObj and hostAttr, that may be returned in the
/// domain info response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainNsList {
    /// List of &lt;hostObj&gt; ns elements
    #[serde(rename = "hostObj")]
    pub host_obj: Option<Vec<StringValue>>,
    /// List of &lt;hostAttr&gt; ns elements
    pub host_attr: Option<Vec<HostAttr>>,
}

/// Type that represents the &lt;infData&gt; tag for domain info response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The domain name
    pub name: StringValue,
    /// The domain ROID
    pub roid: StringValue,
    /// The list of domain statuses
    #[serde(rename = "status")]
    pub statuses: Vec<DomainStatus>,
    /// The domain registrant
    pub registrant: StringValue,
    /// The list of domain contacts
    #[serde(rename = "contact")]
    pub contacts: Vec<DomainContact>,
    /// The list of domain nameservers
    #[serde(rename = "ns")]
    pub ns: Option<DomainNsList>,
    /// The list of domain hosts
    #[serde(rename = "host")]
    pub hosts: Option<Vec<StringValue>>,
    /// The epp user who owns the domain
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// The epp user who created the domain
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    /// The domain creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The epp user who last updated the domain
    #[serde(rename = "upID")]
    pub updater_id: StringValue,
    /// The domain last updated date
    #[serde(rename = "upDate")]
    pub updated_at: StringValue,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
    /// The domain transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    /// The domain auth info
    #[serde(rename = "authInfo")]
    pub auth_info: Option<DomainAuthInfo>,
}

/// Type that represents the &lt;resData&gt; tag for domain info response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoResponse {
    /// Data under the &lt;resData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: DomainInfoResponseData,
}
