//! Types for EPP contact check request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for contact &lt;check&gt; command
///
/// ## Usage
///
/// ```rust
/// use epp_client::EppClient;
/// use epp_client::epp::{EppContactCheck, EppContactCheckResponse};
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
///     // Create an EppContactCheck instance
///     let contact_check = EppContactCheck::new(
///         vec!["epp-client-c1", "epp-client-c2"],
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppContactCheckResponse
///     let response = client.transact::<_, EppContactCheckResponse>(&contact_check).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppContactCheck = EppObject<Command<ContactCheck>>;

/// Type that represents the &lt;check&gt; command for contact transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactList {
    /// The XML namespace for the contact &lt;check&gt;
    xmlns: String,
    /// The list of contact ids to check for availability
    #[serde(rename = "id")]
    pub contact_ids: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// The &lt;command&gt; type for contact check command
pub struct ContactCheck {
    /// The &lt;check&gt; tag for the contact check command
    #[serde(rename = "check")]
    list: ContactList,
}

impl EppContactCheck {
    /// Creates an EppObject corresponding to the &lt;epp&gt; tag with data for a contact check request
    pub fn new(contact_ids: Vec<&str>, client_tr_id: &str) -> EppContactCheck {
        let contact_ids = contact_ids
            .iter()
            .filter_map(|d| Some(d.to_string_value()))
            .collect::<Vec<StringValue>>();

        let contact_check = ContactCheck {
            list: ContactList {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                contact_ids: contact_ids,
            },
        };

        EppObject::build(Command::<ContactCheck>::new(contact_check, client_tr_id))
    }
}
