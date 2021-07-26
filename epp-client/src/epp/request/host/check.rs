//! Types for EPP host check request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for host <check> command
///
/// ## Usage
///
/// ```rust
/// use epp_client::EppClient;
/// use epp_client::epp::{EppHostCheck, EppHostCheckResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create an instance of EppClient, specifying the name of the registry as in
///     // the config file
///     let mut client = match EppClient::new("verisign").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an EppHostCheck instance
///     let host_check = EppHostCheck::new(
///         vec!["ns1.eppdev-101.com", "ns2.eppdev-101.com"],
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppHostCheckResponse
///     let response = client.transact::<_, EppHostCheckResponse>(&host_check).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppHostCheck = EppObject<Command<HostCheck>>;

/// Type for data under the host <check> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostList {
    /// XML namespace for host commands
    xmlns: String,
    /// List of hosts to be checked for availability
    #[serde(rename = "name")]
    pub hosts: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML <check> command for hosts
pub struct HostCheck {
    /// The instance holding the list of hosts to be checked
    #[serde(rename = "check")]
    list: HostList,
}

impl EppHostCheck {
    /// Creates a new EppObject for host check corresponding to the <epp> tag in EPP XML
    pub fn new(hosts: Vec<&str>, client_tr_id: &str) -> EppHostCheck {
        let hosts = hosts
            .iter()
            .filter_map(|d| Some(d.to_string_value()))
            .collect::<Vec<StringValue>>();

        let host_check = HostCheck {
            list: HostList {
                xmlns: EPP_HOST_XMLNS.to_string(),
                hosts: hosts,
            },
        };

        EppObject::build(Command::<HostCheck> {
            command: host_check,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
