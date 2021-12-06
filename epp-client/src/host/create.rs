//! Types for EPP host create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, HostAddr, NoExtension, StringValue};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostCreate<E> {
    request: HostCreateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for HostCreate<E> {
    type Input = HostCreateRequest;
    type Output = HostCreateResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for host &lt;create&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::common::HostAddr;
/// use epp_client::host::create::HostCreate;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
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
///     // Create a vector of IP addresses to assign to the host
///     let addresses = vec![
///         HostAddr::new("v4", "29.245.122.14"),
///         HostAddr::new("v6", "2404:6800:4001:801::200e"),
///     ];
///
///     // Create an HostCreate instance
///     let host_create = HostCreate::<NoExtension>::new("ns1.eppdev-101.com", addresses);
///
///     // send it to the registry and receive a response of type HostCreateResponse
///     let response = client.transact(host_create, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> HostCreate<E> {
    pub fn new(host: &str, addresses: Vec<HostAddr>) -> HostCreate<NoExtension> {
        HostCreate {
            request: HostCreateRequest {
                host: HostCreateRequestData {
                    xmlns: XMLNS.to_string(),
                    name: host.into(),
                    addresses: Some(addresses),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostCreate<F> {
        HostCreate {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for data under the host &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host to be created
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
    /// The list of IP addresses for the host
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for hosts
pub struct HostCreateRequest {
    /// The instance holding the data for the host to be created
    #[serde(rename = "host:create", alias = "create")]
    host: HostCreateRequestData,
}

// Response

/// Type that represents the &lt;creData&gt; tag for host create response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// The host name
    pub name: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: HostCreateData,
}
