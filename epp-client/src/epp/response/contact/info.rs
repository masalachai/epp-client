//! Types for EPP contact info response

use serde::{Deserialize, Serialize};

use crate::epp::object::data::{AuthInfo, ContactStatus, Phone, PostalInfo};
use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML contact info response
pub type EppContactInfoResponse = EppObject<CommandResponse<ContactInfoResult>>;

/// Type that represents the &lt;infData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// XML schema location for contact response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
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
    pub auth_info: Option<AuthInfo>,
}

/// Type that represents the &lt;resData&gt; tag for contact info response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoResult {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: ContactInfoData,
}
