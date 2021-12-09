//! Types for EPP contact info request

use super::XMLNS;
use crate::common::{ContactAuthInfo, ContactStatus, NoExtension, Phone, PostalInfo, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for ContactInfo {}

impl Command for ContactInfo {
    type Response = ContactInfoResponse;
    const COMMAND: &'static str = "info";
}

// Request

/// Type for elements under the contact &lt;info&gt; tag
#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;info&gt; command for contacts
pub struct ContactInfo {
    /// Data for &lt;info&gt; command for contact
    #[serde(rename = "contact:info", alias = "info")]
    info: ContactInfoRequestData,
}

impl ContactInfo {
    pub fn new(id: &str, auth_password: &str) -> ContactInfo {
        Self {
            info: ContactInfoRequestData {
                xmlns: XMLNS.to_string(),
                id: id.into(),
                auth_info: ContactAuthInfo::new(auth_password),
            },
        }
    }
}

// Response

/// Type that represents the &lt;infData&gt; tag for contact check response
#[derive(Deserialize, Debug)]
pub struct ContactInfoData {
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
#[derive(Deserialize, Debug)]
pub struct ContactInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: ContactInfoData,
}
