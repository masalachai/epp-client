//! Types for EPP host info request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
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
/// use epp_client::epp::{EppHostInfo, EppHostInfoResponse};
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
///     client.close().await.unwrap();
/// }
/// ```
pub type EppHostInfo = EppObject<Command<HostInfo>>;

/// Type for data under the host &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoData {
    /// XML namespace for host commands
    xmlns: String,
    /// The name of the host to be queried
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for hosts
pub struct HostInfo {
    /// The instance holding the data for the host query
    #[serde(rename = "info")]
    info: HostInfoData,
}

impl EppHostInfo {
    /// Creates a new EppObject for host info corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppHostInfo {
        EppObject::build(Command::<HostInfo>::new(
            HostInfo {
                info: HostInfoData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                },
            },
            client_tr_id,
        ))
    }
}
