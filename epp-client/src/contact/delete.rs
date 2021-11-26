//! Types for EPP contact delete request

use epp_client_macros::*;

use crate::common::{ElementName, EppObject, StringValue};
use crate::contact::EPP_CONTACT_XMLNS;
use crate::request::Command;
use crate::response::EppCommandResponse;
use serde::{Deserialize, Serialize};

/// Type for the &lt;epp&gt; request for contact &lt;delete&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::contact::delete::{EppContactDelete, EppContactDeleteResponse};
/// use epp_client::generate_client_tr_id;
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
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppContactDelete = EppObject<Command<ContactDeleteRequest>>;

impl EppContactDelete {
    /// Creates a new EppObject for contact delete corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(id: &str, client_tr_id: &str) -> EppContactDelete {
        let contact_delete = ContactDeleteRequest {
            contact: ContactDeleteRequestData {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.into(),
            },
        };

        EppObject::build(Command::<ContactDeleteRequest>::new(
            contact_delete,
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML contact delete response
pub type EppContactDeleteResponse = EppCommandResponse;

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDeleteRequestData {
    /// XML namespace for the &lt;delete&gt; command for contacts
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// The id of the contact to be deleted
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDeleteRequest {
    #[serde(rename = "contact:delete", alias = "delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequestData,
}
