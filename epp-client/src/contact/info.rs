//! Types for EPP contact info request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{
    ContactAuthInfo, ContactStatus, ElementName, NoExtension, Phone, PostalInfo, StringValue,
};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactInfo<E> {
    request: ContactInfoRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for ContactInfo<E> {
    type Input = ContactInfoRequest;
    type Output = ContactInfoResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> ContactInfo<E> {
    pub fn new(id: &str, auth_password: &str) -> ContactInfo<NoExtension> {
        ContactInfo {
            request: ContactInfoRequest {
                info: ContactInfoRequestData {
                    xmlns: XMLNS.to_string(),
                    id: id.into(),
                    auth_info: ContactAuthInfo::new(auth_password),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactInfo<F> {
        ContactInfo {
            request: self.request,
            extension: Some(extension),
        }
    }
}

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
