//! Types for EPP host create request

use epp_client_macros::*;

use crate::epp::object::data::HostAddr;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
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
/// use epp_client::epp::object::data::HostAddr;
/// use epp_client::epp::{EppHostCreate, EppHostCreateResponse};
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
///     client.close().await.unwrap();
/// }
/// ```
pub type EppHostCreate = EppObject<Command<HostCreate>>;

/// Type for data under the host &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    /// XML namespace for host commands
    xmlns: String,
    /// The name of the host to be created
    pub name: StringValue,
    /// The list of IP addresses for the host
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for hosts
pub struct HostCreate {
    /// The instance holding the data for the host to be created
    #[serde(rename = "create")]
    host: HostCreateData,
}

impl EppHostCreate {
    /// Creates a new EppObject for host create corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(host: &str, addresses: Vec<HostAddr>, client_tr_id: &str) -> EppHostCreate {
        let host_create = HostCreate {
            host: HostCreateData {
                xmlns: EPP_HOST_XMLNS.to_string(),
                name: host.to_string_value(),
                addresses: Some(addresses),
            },
        };

        EppObject::build(Command::<HostCreate>::new(host_create, client_tr_id))
    }
}
