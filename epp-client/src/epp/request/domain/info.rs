//! Types for EPP domain info request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;info&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppDomainInfo, EppDomainInfoResponse};
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
///     // Create an EppDomainInfo instance
///     let domain_info = EppDomainInfo::new("eppdev-100.com", generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppDomainInfoResponse
///     let response = client.transact::<_, EppDomainInfoResponse>(&domain_info).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppDomainInfo = EppObject<Command<DomainInfo>>;

/// Type for data under the &lt;name&gt; element tag for the domain &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    /// The hosts attribute. Default value is "all"
    hosts: String,
    /// The name of the domain
    #[serde(rename = "$value")]
    name: String,
}

/// Type for &lt;name&gt; element under the domain &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoData {
    /// XML namespace for domain commands
    xmlns: String,
    /// The data for the domain to be queried
    #[serde(rename = "name")]
    domain: Domain,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for domains
pub struct DomainInfo {
    /// The data under the &lt;info&gt; tag for domain info
    #[serde(rename = "info")]
    info: DomainInfoData,
}

impl EppDomainInfo {
    /// Creates a new EppObject for domain info corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainInfo {
        EppObject::build(Command::<DomainInfo> {
            command: DomainInfo {
                info: DomainInfoData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domain: Domain {
                        hosts: "all".to_string(),
                        name: name.to_string(),
                    },
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
