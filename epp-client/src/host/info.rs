//! Types for EPP host info request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, HostAddr, HostStatus, NoExtension, StringValue};
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostInfo<E> {
    request: HostInfoRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for HostInfo<E> {
    type Input = HostInfoRequest;
    type Output = HostInfoResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for host &lt;info&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::host::info::HostInfo;
/// use epp_client::generate_client_tr_id;
/// use epp_client::common::NoExtension;
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
///     // Create an HostInfo instance
///     let host_info = HostInfo::<NoExtension>::new("ns2.eppdev-101.com");
///
///     // send it to the registry and receive a response of type HostInfoResponse
///     let response = client.transact(host_info, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> HostInfo<E> {
    pub fn new(name: &str) -> HostInfo<NoExtension> {
        HostInfo {
            request: HostInfoRequest {
                info: HostInfoRequestData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostInfo<F> {
        HostInfo {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for data under the host &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host to be queried
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for hosts
pub struct HostInfoRequest {
    /// The instance holding the data for the host query
    #[serde(rename = "host:info", alias = "info")]
    info: HostInfoRequestData,
}

// Response

/// Type that represents the &lt;infData&gt; tag for host info response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoResponseData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// The host name
    pub name: StringValue,
    /// The host ROID
    pub roid: StringValue,
    /// The list of host statuses
    #[serde(rename = "status")]
    pub statuses: Vec<HostStatus>,
    /// The list of host IP addresses
    #[serde(rename = "addr")]
    pub addresses: Vec<HostAddr>,
    /// The epp user to whom the host belongs
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// THe epp user that created the host
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The epp user that last updated the host
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    /// The host last update date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    /// The host transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
}

/// Type that represents the &lt;resData&gt; tag for host info response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: HostInfoResponseData,
}
