//! Types for EPP host delete request

use epp_client_macros::*;

use crate::common::{ElementName, EppObject, StringValue};
use crate::epp::xml::EPP_HOST_XMLNS;
use crate::request::Command;
use crate::response::EppCommandResponse;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for host &lt;delete&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::host::delete::{EppHostDelete, EppHostDeleteResponse};
/// use epp_client::epp::generate_client_tr_id;
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
///     // Create an EppHostDelete instance
///     let host_delete = EppHostDelete::new("ns2.eppdev-101.com", generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppHostDeleteResponse
///     let response = client.transact::<_, EppHostDeleteResponse>(&host_delete).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppHostDelete = EppObject<Command<HostDeleteRequest>>;

impl EppHostDelete {
    /// Creates a new EppObject for host delete corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppHostDelete {
        EppObject::build(Command::<HostDeleteRequest>::new(
            HostDeleteRequest {
                host: HostDeleteRequestData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.into(),
                },
            },
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML host delete response
pub type EppHostDeleteResponse = EppCommandResponse;

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
