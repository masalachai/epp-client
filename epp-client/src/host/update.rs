//! Types for EPP host update request

use epp_client_macros::*;

use crate::common::{ElementName, HostAddr, HostStatus, NoExtension, StringValue};
use crate::host::EPP_HOST_XMLNS;
use crate::request::{EppExtension, EppRequest};
use crate::response::EppCommandResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostUpdate<E> {
    request: HostUpdateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for HostUpdate<E> {
    type Input = HostUpdateRequest;
    type Output = EppCommandResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for host &lt;update&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::common::{HostAddr, HostStatus};
/// use epp_client::host::update::{HostUpdate, HostAddRemove, HostChangeInfo};
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
///     // Create an HostUpdate instance
///     let mut host_update = HostUpdate::<NoExtension>::new("ns1.eppdev-101.com");
///
///     /// Prepare the add and remove sections for the update
///     let add = HostAddRemove {
///         addresses: Some(vec![
///             HostAddr::new("v4", "177.34.126.17")
///         ]),
///         statuses: None
///     };
///
///     let remove = HostAddRemove {
///         addresses: Some(vec![
///             HostAddr::new("v6", "2404:6800:4001:801::200e")
///         ]),
///         statuses: None,
///     };
///
///     host_update.add(add);
///     host_update.remove(remove);
///
///     // Send a &lt;chg&gt; section as well
///     host_update.info(HostChangeInfo { name: "ns2.eppdev-101.com".into() });
///
///     // send it to the registry and receive a response of type HostUpdateResponse
///     let response = client.transact(host_update, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> HostUpdate<E> {
    pub fn new(name: &str) -> HostUpdate<NoExtension> {
        HostUpdate {
            request: HostUpdateRequest {
                host: HostUpdateRequestData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.into(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostUpdate<F> {
        HostUpdate {
            request: self.request,
            extension: Some(extension),
        }
    }

    /// Sets the data for the &lt;chg&gt; element of the host update
    pub fn info(&mut self, info: HostChangeInfo) {
        self.request.host.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; element of the host update
    pub fn add(&mut self, add: HostAddRemove) {
        self.request.host.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; element of the host update
    pub fn remove(&mut self, remove: HostAddRemove) {
        self.request.host.remove = Some(remove);
    }
}

/// Type for data under the &lt;chg&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostChangeInfo {
    /// The new name for the host
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddRemove {
    /// The IP addresses to be added to or removed from the host
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
    /// The statuses to be added to or removed from the host
    #[serde(rename = "host:status", alias = "status")]
    pub statuses: Option<Vec<HostStatus>>,
}

/// Type for data under the host &lt;update&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostUpdateRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
    /// The IP addresses and statuses to be added to the host
    #[serde(rename = "host:add", alias = "add")]
    add: Option<HostAddRemove>,
    /// The IP addresses and statuses to be removed from the host
    #[serde(rename = "host:rem", alias = "rem")]
    remove: Option<HostAddRemove>,
    /// The host details that need to be updated
    #[serde(rename = "host:chg", alias = "chg")]
    change_info: Option<HostChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for hosts
pub struct HostUpdateRequest {
    /// The instance holding the data for the host to be updated
    #[serde(rename = "host:update", alias = "update")]
    host: HostUpdateRequestData,
}
