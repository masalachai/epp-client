//! Types for EPP host update request

use epp_client_macros::*;

use crate::epp::object::data::{HostAddr, HostStatus};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_HOST_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for host &lt;update&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::object::StringValueTrait;
/// use epp_client::epp::object::data::{HostAddr, HostStatus};
/// use epp_client::epp::{EppHostUpdate, EppHostUpdateResponse, HostAddRemove, HostChangeInfo};
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
///     // Create an EppHostUpdate instance
///     let mut host_update = EppHostUpdate::new("ns1.eppdev-101.com", generate_client_tr_id(&client).as_str());
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
///     host_update.info(HostChangeInfo { name: "ns2.eppdev-101.com".to_string_value() });
///
///     // send it to the registry and receive a response of type EppHostUpdateResponse
///     let response = client.transact::<_, EppHostUpdateResponse>(&host_update).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppHostUpdate = EppObject<Command<HostUpdate>>;

/// Type for data under the &lt;chg&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostChangeInfo {
    /// The new name for the host
    pub name: StringValue,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddRemove {
    /// The IP addresses to be added to or removed from the host
    #[serde(rename = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
    /// The statuses to be added to or removed from the host
    #[serde(rename = "status")]
    pub statuses: Option<Vec<HostStatus>>,
}

/// Type for data under the host &lt;update&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostUpdateData {
    /// XML namespace for host commands
    xmlns: String,
    /// The name of the host
    name: StringValue,
    /// The IP addresses and statuses to be added to the host
    add: Option<HostAddRemove>,
    /// The IP addresses and statuses to be removed from the host
    #[serde(rename = "rem")]
    remove: Option<HostAddRemove>,
    /// The host details that need to be updated
    #[serde(rename = "chg")]
    change_info: Option<HostChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for hosts
pub struct HostUpdate {
    /// The instance holding the data for the host to be updated
    #[serde(rename = "update")]
    host: HostUpdateData,
}

impl EppHostUpdate {
    /// Creates a new EppObject for host update corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppHostUpdate {
        EppObject::build(Command::<HostUpdate>::new(
            HostUpdate {
                host: HostUpdateData {
                    xmlns: EPP_HOST_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id,
        ))
    }

    /// Sets the data for the &lt;chg&gt; element of the host update
    pub fn info(&mut self, info: HostChangeInfo) {
        self.data.command.host.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; element of the host update
    pub fn add(&mut self, add: HostAddRemove) {
        self.data.command.host.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; element of the host update
    pub fn remove(&mut self, remove: HostAddRemove) {
        self.data.command.host.remove = Some(remove);
    }
}
