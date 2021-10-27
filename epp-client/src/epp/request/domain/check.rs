//! Types for EPP domain check request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command
///
/// ## Usage
///
/// ```rust
/// use epp_client::EppClient;
/// use epp_client::epp::{EppDomainCheck, EppDomainCheckResponse};
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
///     // Create an EppDomainCheck instance
///     let domain_check = EppDomainCheck::new(
///         vec!["eppdev-100.com", "eppdev-100.net"],
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainCheckResponse
///     let response = client.transact::<_, EppDomainCheckResponse>(&domain_check).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppDomainCheck = EppObject<Command<DomainCheck>>;

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    /// XML namespace for domain commands
    pub xmlns: String,
    #[serde(rename = "name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheck {
    /// The object holding the list of domains to be checked
    #[serde(rename = "check")]
    list: DomainList,
}

impl EppDomainCheck {
    /// Creates a new EppObject for domain check corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(domains: Vec<&str>, client_tr_id: &str) -> EppDomainCheck {
        let domains = domains
            .iter()
            .map(|d| d.to_string_value())
            .collect::<Vec<StringValue>>();

        let domain_check = DomainCheck {
            list: DomainList {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                domains,
            },
        };

        EppObject::build(Command::<DomainCheck>::new(domain_check, client_tr_id))
    }
}
