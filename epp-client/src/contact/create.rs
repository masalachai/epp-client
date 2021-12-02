//! Types for EPP contact create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ContactAuthInfo, ElementName, NoExtension, Phone, PostalInfo, StringValue};
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactCreate<E> {
    request: ContactCreateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for ContactCreate<E> {
    type Input = ContactCreateRequest;
    type Output = ContactCreateResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for contact &lt;create&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::common::{Address, Phone, PostalInfo};
/// use epp_client::contact::create::ContactCreate;
/// use epp_client::generate_client_tr_id;
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
///     // Create the address, postal_info, voice instances
///     let street = vec!["58", "Orchid Road"];
///     let address = Address::new(&street, "New York", "New York", "392374", "US");
///     let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
///     let mut voice = Phone::new("+1.47237942");
///     voice.set_extension("123");
///     let mut fax = Phone::new("+1.86698799");
///     fax.set_extension("677");
///
///     // Create an ContactCreate instance
///     let mut contact_create = ContactCreate::<NoExtension>::new(
///         "eppdev-contact-100",
///         "contact@eppdev.net",
///         postal_info,
///         voice,
///         "epP4uthd#v"
///     );
///     contact_create.set_fax(fax);
///
///     // send it to the registry and receive a response of type ContactCreateResponse
///     let response = client.transact(contact_create, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> ContactCreate<E> {
    pub fn new(
        id: &str,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
    ) -> ContactCreate<NoExtension> {
        ContactCreate {
            request: ContactCreateRequest {
                contact: Contact {
                    xmlns: XMLNS.to_string(),
                    id: id.into(),
                    postal_info,
                    voice,
                    fax: None,
                    email: email.into(),
                    auth_info: ContactAuthInfo::new(auth_password),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactCreate<F> {
        ContactCreate {
            request: self.request,
            extension: Some(extension),
        }
    }

    /// Sets the &lt;fax&gt; data for the request
    pub fn set_fax(&mut self, fax: Phone) {
        self.request.contact.fax = Some(fax);
    }
}

// Request

/// Type for elements under the contact &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    /// XML namespace for contact commands
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// Contact &lt;id&gt; tag
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
    /// Contact &lt;postalInfo&gt; tag
    #[serde(rename = "contact:postalInfo", alias = "postalInfo")]
    postal_info: PostalInfo,
    /// Contact &lt;voice&gt; tag
    #[serde(rename = "contact:voice", alias = "voice")]
    voice: Phone,
    /// Contact &lt;fax&gt; tag,
    #[serde(rename = "contact:fax", alias = "fax")]
    fax: Option<Phone>,
    /// Contact &lt;email&gt; tag
    #[serde(rename = "contact:email", alias = "email")]
    email: StringValue,
    /// Contact &lt;authInfo&gt; tag
    #[serde(rename = "contact:authInfo", alias = "authInfo")]
    auth_info: ContactAuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for contacts
pub struct ContactCreateRequest {
    /// Data for &lt;create&gt; command for contact
    #[serde(rename = "contact:create", alias = "create")]
    pub contact: Contact,
}

// Response

/// Type that represents the &lt;creData&gt; tag for contact create response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// The contact id
    pub id: StringValue,
    #[serde(rename = "crDate")]
    /// The contact creation date
    pub created_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for contact create response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: ContactCreateData,
}
