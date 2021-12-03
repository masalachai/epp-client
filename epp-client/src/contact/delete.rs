//! Types for EPP contact delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, EppRequest};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactDelete<E> {
    request: ContactDeleteRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for ContactDelete<E> {
    type Input = ContactDeleteRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type for the &lt;epp&gt; request for contact &lt;delete&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::contact::delete::ContactDelete;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", &None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an ContactDelete instance
///     let contact_delete = ContactDelete::<NoExtension>::new(
///         "eppdev-contact-100"
///     );
///
///     // send it to the registry and receive a response of type ContactDeleteResponse
///     let response = client.transact(contact_delete, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> ContactDelete<E> {
    pub fn new(id: &str) -> ContactDelete<NoExtension> {
        ContactDelete {
            request: ContactDeleteRequest {
                contact: ContactDeleteRequestData {
                    xmlns: XMLNS.to_string(),
                    id: id.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactDelete<F> {
        ContactDelete {
            request: self.request,
            extension: Some(extension),
        }
    }
}

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
