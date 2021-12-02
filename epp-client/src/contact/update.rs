//! Types for EPP contact create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{
    ContactAuthInfo, ContactStatusWithEnum, ElementName, NoExtension, Phone, PostalInfo,
    StringValue,
};
use crate::request::{EppExtension, EppRequest};
use crate::response::EppCommandResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactUpdate<E> {
    request: ContactUpdateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for ContactUpdate<E> {
    type Input = ContactUpdateRequest;
    type Output = EppCommandResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for contact &lt;update&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::contact::update::ContactUpdate;
/// use epp_client::generate_client_tr_id;
/// use epp_client::common::{ContactStatusWithEnum, Status};
/// use epp_client::common::NoExtension;
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
///     // Create an ContactUpdate instance
///     let mut contact_update = ContactUpdate::<NoExtension>::new(
///         "eppdev-contact-100"
///     );
///
///     let add_statuses = vec![
///         ContactStatusWithEnum {
///             status: Status::ClientTransferProhibited,
///         }
///     ];
///
///     contact_update.add(add_statuses);
///
///     // send it to the registry and receive a response of type ContactUpdateResponse
///     let response = client.transact(contact_update, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> ContactUpdate<E> {
    pub fn new(id: &str) -> ContactUpdate<NoExtension> {
        ContactUpdate {
            request: ContactUpdateRequest {
                contact: ContactUpdateRequestData {
                    xmlns: XMLNS.to_string(),
                    id: id.into(),
                    add_statuses: None,
                    remove_statuses: None,
                    change_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactUpdate<F> {
        ContactUpdate {
            request: self.request,
            extension: Some(extension),
        }
    }

    /// Sets the data for the &lt;chg&gt; tag for the contact update request
    pub fn set_info(
        &mut self,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
    ) {
        self.request.contact.change_info = Some(ContactChangeInfo {
            email: Some(email.into()),
            postal_info: Some(postal_info),
            voice: Some(voice),
            auth_info: Some(ContactAuthInfo::new(auth_password)),
            fax: None,
        });
    }

    /// Sets the data for the &lt;fax&gt; tag under &lt;chg&gt; for the contact update request
    pub fn set_fax(&mut self, fax: Phone) {
        if let Some(info) = &mut self.request.contact.change_info {
            info.fax = Some(fax)
        }
    }

    /// Sets the data for the &lt;add&gt; tag for the contact update request
    pub fn add(&mut self, statuses: Vec<ContactStatusWithEnum>) {
        self.request.contact.add_statuses = Some(StatusList { status: statuses });
    }

    /// Sets the data for the &lt;rem&gt; tag for the contact update request
    pub fn remove(&mut self, statuses: Vec<ContactStatusWithEnum>) {
        self.request.contact.remove_statuses = Some(StatusList { status: statuses });
    }
}

/// Type for elements under the &lt;chg&gt; tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactChangeInfo {
    #[serde(rename = "contact:postalInfo", alias = "postalInfo")]
    postal_info: Option<PostalInfo>,
    #[serde(rename = "contact:voice", alias = "voice")]
    voice: Option<Phone>,
    #[serde(rename = "contact:fax", alias = "fax")]
    fax: Option<Phone>,
    #[serde(rename = "contact:email", alias = "email")]
    email: Option<StringValue>,
    #[serde(rename = "contact:authInfo", alias = "authInfo")]
    auth_info: Option<ContactAuthInfo>,
}

/// Type for list of elements of the &lt;status&gt; tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusList {
    #[serde(rename = "contact:status", alias = "status")]
    status: Vec<ContactStatusWithEnum>,
}

/// Type for elements under the contact &lt;update&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactUpdateRequestData {
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
    #[serde(rename = "contact:add", alias = "add")]
    add_statuses: Option<StatusList>,
    #[serde(rename = "contact:rem", alias = "rem")]
    remove_statuses: Option<StatusList>,
    #[serde(rename = "contact:chg", alias = "chg")]
    change_info: Option<ContactChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for contacts
pub struct ContactUpdateRequest {
    /// The data under the &lt;update&gt; tag for the contact update
    #[serde(rename = "contact:update", alias = "update")]
    contact: ContactUpdateRequestData,
}
