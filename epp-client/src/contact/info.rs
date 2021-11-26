//! Types for EPP contact info request

use epp_client_macros::*;

use crate::common::{
    ContactAuthInfo, ContactStatus, ElementName, EppObject, Phone, PostalInfo, StringValue,
};
use crate::contact::EPP_CONTACT_XMLNS;
use crate::request::Command;
use crate::response::CommandResponse;
use serde::{Deserialize, Serialize};

/// Type for the &lt;epp&gt; request for contact &lt;info&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::contact::info::{EppContactInfo, EppContactInfoResponse};
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
///     // Create an EppContactInfo instance
///     let contact_info = EppContactInfo::new(
///         "eppdev-contact-100",
///         "epP4uthd#v",
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppContactInfoResponse
///     let response = client.transact::<_, EppContactInfoResponse>(&contact_info).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppContactInfo = EppObject<Command<ContactInfoRequest>>;

impl EppContactInfo {
    /// Creates a new EppObject for contact info corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(id: &str, auth_password: &str, client_tr_id: &str) -> EppContactInfo {
        let contact_info = ContactInfoRequest {
            info: ContactInfoRequestData {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.into(),
                auth_info: ContactAuthInfo::new(auth_password),
            },
        };

        EppObject::build(Command::<ContactInfoRequest>::new(
            contact_info,
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML contact info response
pub type EppContactInfoResponse = EppObject<CommandResponse<ContactInfoResponse>>;

// Request

/// Type for elements under the contact &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoRequestData {
    /// XML namespace for contact commands
    #[serde(rename = "xmlns:contact", alias = "contact")]
    xmlns: String,
    /// The contact id for the info command
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
    /// The &lt;authInfo&gt; data
    #[serde(rename = "contact:authInfo", alias = "authInfo")]
    auth_info: ContactAuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for contacts
pub struct ContactInfoRequest {
    /// Data for &lt;info&gt; command for contact
    #[serde(rename = "contact:info", alias = "info")]
    info: ContactInfoRequestData,
}

// Response

/// Type that represents the &lt;infData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// The contact id
    pub id: StringValue,
    /// The contact ROID
    pub roid: StringValue,
    /// The list of contact statuses
    #[serde(rename = "status")]
    pub statuses: Vec<ContactStatus>,
    /// The postal info for the contact
    #[serde(rename = "postalInfo")]
    pub postal_info: PostalInfo,
    /// The voice data for the contact
    pub voice: Phone,
    /// The fax data for the contact
    pub fax: Option<Phone>,
    /// The email for the contact
    pub email: StringValue,
    /// The epp user to whom the contact belongs
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// The epp user who created the contact
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    /// The creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The epp user who last updated the contact
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    /// The last update date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    /// The contact transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    /// The contact auth info
    #[serde(rename = "authInfo")]
    pub auth_info: Option<ContactAuthInfo>,
}

/// Type that represents the &lt;resData&gt; tag for contact info response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: ContactInfoData,
}
