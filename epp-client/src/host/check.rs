//! Types for EPP host check request

use std::fmt::Debug;

use epp_client_macros::*;

use crate::common::{ElementName, NoExtension, StringValue};
use crate::host::EPP_HOST_XMLNS;
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostCheck<E> {
    request: HostCheckRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for HostCheck<E> {
    type Input = HostCheckRequest;
    type Output = HostCheckResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for host &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::host::check::HostCheck;
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
///     // Create an HostCheck instance
///     let host_check = HostCheck::<NoExtension>::new(
///         &["ns1.eppdev-101.com", "ns2.eppdev-101.com"]
///     );
///
///     // send it to the registry and receive a response of type HostCheckResponse
///     let response = client.transact(host_check, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> HostCheck<E> {
    pub fn new(hosts: &[&str]) -> HostCheck<NoExtension> {
        let hosts = hosts.iter().map(|&d| d.into()).collect();

        HostCheck {
            request: HostCheckRequest {
                list: HostList {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    hosts,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostCheck<F> {
        HostCheck {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for data under the host &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostList {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// List of hosts to be checked for availability
    #[serde(rename = "host:name", alias = "name")]
    pub hosts: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for hosts
pub struct HostCheckRequest {
    /// The instance holding the list of hosts to be checked
    #[serde(rename = "host:check", alias = "check")]
    list: HostList,
}

// Response

/// Type that represents the &lt;name&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAvailable {
    /// The host name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The host (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub host: HostAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub host_list: Vec<HostCheckDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: HostCheckData,
}
