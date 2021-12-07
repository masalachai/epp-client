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

#[cfg(test)]
mod tests {
    use super::ContactInfo;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn contact_info() {
        let xml = get_xml("response/contact/info.xml").unwrap();
        let object = ContactInfo::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();
        let fax = result.info_data.fax.as_ref().unwrap();
        let voice_ext = result.info_data.voice.extension.as_ref().unwrap();
        let fax_ext = fax.extension.as_ref().unwrap();
        let auth_info = result.info_data.auth_info.as_ref().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.info_data.id, "eppdev-contact-3".into());
        assert_eq!(result.info_data.roid, "UNDEF-ROID".into());
        assert_eq!(result.info_data.statuses[0].status, "ok".to_string());
        assert_eq!(result.info_data.postal_info.info_type, "loc".to_string());
        assert_eq!(result.info_data.postal_info.name, "John Doe".into());
        assert_eq!(
            result.info_data.postal_info.organization,
            "Acme Widgets".into()
        );
        assert_eq!(result.info_data.postal_info.address.street[0], "58".into());
        assert_eq!(
            result.info_data.postal_info.address.street[1],
            "Orchid Road".into()
        );
        assert_eq!(result.info_data.postal_info.address.city, "Paris".into());
        assert_eq!(
            result.info_data.postal_info.address.province,
            "Paris".into()
        );
        assert_eq!(
            result.info_data.postal_info.address.postal_code,
            "392374".into()
        );
        assert_eq!(result.info_data.postal_info.address.country.alpha2, "FR");
        assert_eq!(result.info_data.voice.number, "+33.47237942".to_string());
        assert_eq!(*voice_ext, "123".to_string());
        assert_eq!(fax.number, "+33.86698799".to_string());
        assert_eq!(*fax_ext, "243".to_string());
        assert_eq!(result.info_data.email, "contact@eppdev.net".into());
        assert_eq!(result.info_data.client_id, "eppdev".into());
        assert_eq!(result.info_data.creator_id, "SYSTEM".into());
        assert_eq!(result.info_data.created_at, "2021-07-23T13:09:09.0Z".into());
        assert_eq!(
            *(result.info_data.updater_id.as_ref().unwrap()),
            "SYSTEM".into()
        );
        assert_eq!(
            *(result.info_data.updated_at.as_ref().unwrap()),
            "2021-07-23T13:09:09.0Z".into()
        );
        assert_eq!((*auth_info).password, "eppdev-387323".into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
