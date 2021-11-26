//! Types for EPP host info request

use epp_client_macros::*;

use crate::epp::object::data::{HostAddr, HostStatus};
use crate::epp::object::{ElementName, EppObject, StringValue};
use crate::epp::request::Command;
use crate::epp::response::CommandResponse;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for host &lt;info&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::host::info::{EppHostInfo, EppHostInfoResponse};
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
///     // Create an EppHostCreate instance
///     let host_info = EppHostInfo::new("ns2.eppdev-101.com", generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppHostInfoResponse
///     let response = client.transact::<_, EppHostInfoResponse>(&host_info).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppHostInfo = EppObject<Command<HostInfoRequest>>;

impl EppHostInfo {
    /// Creates a new EppObject for host info corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppHostInfo {
        EppObject::build(Command::<HostInfoRequest>::new(
            HostInfoRequest {
                info: HostInfoRequestData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.into(),
                },
            },
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML host info response
pub type EppHostInfoResponse = EppObject<CommandResponse<HostInfoResponse>>;

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
