//! Types for EPP host create request

use epp_client_macros::*;

use crate::common::{ElementName, EppObject, HostAddr, StringValue};
use crate::host::EPP_HOST_XMLNS;
use crate::request::Command;
use crate::response::CommandResponse;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for host &lt;create&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::common::HostAddr;
/// use epp_client::host::create::{EppHostCreate, EppHostCreateResponse};
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
///     // Create a vector of IP addresses to assign to the host
///     let addresses = vec![
///         HostAddr::new("v4", "29.245.122.14"),
///         HostAddr::new("v6", "2404:6800:4001:801::200e"),
///     ];
///
///     // Create an EppHostCreate instance
///     let host_create = EppHostCreate::new("ns1.eppdev-101.com", addresses, generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppHostCreateResponse
///     let response = client.transact::<_, EppHostCreateResponse>(&host_create).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppHostCreate = EppObject<Command<HostCreateRequest>>;

impl EppHostCreate {
    /// Creates a new EppObject for host create corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(host: &str, addresses: Vec<HostAddr>, client_tr_id: &str) -> EppHostCreate {
        let host_create = HostCreateRequest {
            host: HostCreateRequestData {
                xmlns: EPP_HOST_XMLNS.to_string(),
                name: host.into(),
                addresses: Some(addresses),
            },
        };

        EppObject::build(Command::<HostCreateRequest>::new(host_create, client_tr_id))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML host create response
pub type EppHostCreateResponse = EppObject<CommandResponse<HostCreateResponse>>;

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
