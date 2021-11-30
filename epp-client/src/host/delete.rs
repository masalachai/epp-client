//! Types for EPP host delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, EppRequest};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostDelete<E> {
    request: HostDeleteRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for HostDelete<E> {
    type Input = HostDeleteRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for host &lt;delete&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::host::delete::HostDelete;
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
///     let login = Login::<NoExtension>::new("username", "password", &None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an HostDelete instance
///     let host_delete = HostDelete::<NoExtension>::new("ns2.eppdev-101.com");
///
///     // send it to the registry and receive a response of type HostDeleteResponse
///     let response = client.transact(host_delete, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> HostDelete<E> {
    pub fn new(name: &str) -> HostDelete<NoExtension> {
        HostDelete {
            request: HostDeleteRequest {
                host: HostDeleteRequestData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostDelete<F> {
        HostDelete {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type for data under the host &lt;delete&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostDeleteRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The host to be deleted
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML &lt;delete&gt; command for hosts
pub struct HostDeleteRequest {
    /// The instance holding the data for the host to be deleted
    #[serde(rename = "host:delete", alias = "delete")]
    host: HostDeleteRequestData,
}
