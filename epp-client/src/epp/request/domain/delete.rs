//! Types for EPP domain delete request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;delete&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppDomainDelete, EppDomainDeleteResponse};
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
///     // Create an EppDomainDelete instance
///     let mut domain_delete = EppDomainDelete::new("eppdev-100.com", generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppDomainDeleteResponse
///     let response = client.transact::<_, EppDomainDeleteResponse>(&domain_delete).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppDomainDelete = EppObject<Command<DomainDelete>>;

/// Type for &lt;name&gt; element under the domain &lt;delete&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainDeleteData {
    /// XML namespace for domain commands
    xmlns: String,
    /// The domain to be deleted
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML &lt;delete&gt; command for domains
pub struct DomainDelete {
    /// The data under the &lt;delete&gt; tag for domain deletion
    #[serde(rename = "delete")]
    domain: DomainDeleteData,
}

impl EppDomainDelete {
    /// Creates a new EppObject for domain delete corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainDelete {
        EppObject::build(Command::<DomainDelete>::new(
            DomainDelete {
                domain: DomainDeleteData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                },
            },
            client_tr_id,
        ))
    }
}
