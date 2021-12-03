use std::fmt::Debug;

/// Types for EPP contact check request
use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, EppRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactCheck<E> {
    request: ContactCheckRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for ContactCheck<E> {
    type Input = ContactCheckRequest;
    type Output = ContactCheckResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for contact &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::contact::check::ContactCheck;
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
///     // Create an instance of EppClient, passing the config
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an ContactCheck instance
///     let contact_check = ContactCheck::<NoExtension>::new(
///         &["epp-client-c1", "epp-client-c2"]
///     );
///
///     // send it to the registry and receive a response of type ContactCheckResponse
///
///     let response = client.transact(contact_check, "transaction-id").await.unwrap();
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> ContactCheck<E> {
    pub fn new(contact_ids: &[&str]) -> ContactCheck<NoExtension> {
        let contact_ids = contact_ids
            .iter()
            .map(|&d| d.into())
            .collect::<Vec<StringValue>>();

        ContactCheck {
            request: ContactCheckRequest {
                list: ContactList {
                    xmlns: XMLNS.to_string(),
                    contact_ids,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactCheck<F> {
        ContactCheck {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type that represents the &lt;check&gt; command for contact transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactList {
    /// The XML namespace for the contact &lt;check&gt;
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// The list of contact ids to check for availability
    #[serde(rename = "contact:id", alias = "id")]
    pub contact_ids: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// The &lt;command&gt; type for contact check command
pub struct ContactCheckRequest {
    /// The &lt;check&gt; tag for the contact check command
    #[serde(rename = "contact:check", alias = "check")]
    list: ContactList,
}

// Response

/// Type that represents the &lt;id&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactAvailable {
    /// The text of the &lt;id&gt; tag
    #[serde(rename = "$value")]
    pub id: StringValue,
    /// The avail attr on the &lt;id&gt; tag
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResponseDataItem {
    /// Data under the &lt;id&gt; tag
    #[serde(rename = "id")]
    pub contact: ContactAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResponseData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub contact_list: Vec<ContactCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: ContactCheckResponseData,
}
