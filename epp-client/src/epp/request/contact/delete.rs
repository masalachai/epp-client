//! Types for EPP contact delete request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type for the &lt;epp&gt; request for contact &lt;delete&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppContactDelete, EppContactDeleteResponse};
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
///     // Create an EppContactDelete instance
///     let contact_delete = EppContactDelete::new(
///         "eppdev-contact-100",
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppContactDeleteResponse
///     let response = client.transact::<_, EppContactDeleteResponse>(&contact_delete).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppContactDelete = EppObject<Command<ContactDelete>>;

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDeleteData {
    /// XML namespace for the &lt;delete&gt; command for contacts
    xmlns: String,
    /// The id of the contact to be deleted
    id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDelete {
    #[serde(rename = "delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteData,
}

impl EppContactDelete {
    /// Creates a new EppObject for contact delete corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(id: &str, client_tr_id: &str) -> EppContactDelete {
        let contact_delete = ContactDelete {
            contact: ContactDeleteData {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.to_string_value(),
            },
        };

        EppObject::build(Command::<ContactDelete>::new(contact_delete, client_tr_id))
    }
}
