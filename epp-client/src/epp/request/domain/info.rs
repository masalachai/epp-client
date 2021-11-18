//! Types for EPP domain info request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;info&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppDomainInfo, EppDomainInfoResponse};
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
///     // Create an EppDomainInfo instance
///     let domain_info = EppDomainInfo::new("eppdev-100.com", generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppDomainInfoResponse
///     let response = client.transact::<_, EppDomainInfoResponse>(&domain_info).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
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
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The data for the domain to be queried
    #[serde(rename = "domain:name", alias = "name")]
    domain: Domain,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for domains
pub struct DomainInfo {
    /// The data under the &lt;info&gt; tag for domain info
    #[serde(rename = "domain:info", alias = "info")]
    info: DomainInfoData,
}

impl EppDomainInfo {
    /// Creates a new EppObject for domain info corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainInfo {
        EppObject::build(Command::<DomainInfo>::new(
            DomainInfo {
                info: DomainInfoData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domain: Domain {
                        hosts: "all".to_string(),
                        name: name.to_string(),
                    },
                },
            },
            client_tr_id,
        ))
    }
}
