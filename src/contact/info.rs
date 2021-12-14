//! Types for EPP contact info request

use super::{ContactAuthInfo, Phone, PostalInfo, XMLNS};
use crate::common::{NoExtension, ObjectStatus, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl<'a> Transaction<NoExtension> for ContactInfo<'a> {}

impl<'a> Command for ContactInfo<'a> {
    type Response = ContactInfoResponse;
    const COMMAND: &'static str = "info";
}

// Request

/// Type for elements under the contact &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct ContactInfoRequestData<'a> {
    /// XML namespace for contact commands
    #[serde(rename = "xmlns:contact")]
    xmlns: &'a str,
    /// The contact id for the info command
    #[serde(rename = "contact:id")]
    id: StringValue<'a>,
    /// The &lt;authInfo&gt; data
    #[serde(rename = "contact:authInfo")]
    auth_info: ContactAuthInfo<'a>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;info&gt; command for contacts
pub struct ContactInfo<'a> {
    /// Data for &lt;info&gt; command for contact
    #[serde(rename = "contact:info")]
    info: ContactInfoRequestData<'a>,
}

impl<'a> ContactInfo<'a> {
    pub fn new(id: &'a str, auth_password: &'a str) -> ContactInfo<'a> {
        Self {
            info: ContactInfoRequestData {
                xmlns: XMLNS,
                id: id.into(),
                auth_info: ContactAuthInfo::new(auth_password),
            },
        }
    }
}

// Response

/// Type that represents the &lt;infData&gt; tag for contact check response
#[derive(Deserialize, Debug)]
pub struct ContactInfoData<'a> {
    /// The contact id
    pub id: StringValue<'a>,
    /// The contact ROID
    pub roid: StringValue<'a>,
    /// The list of contact statuses
    #[serde(rename = "status")]
    pub statuses: Vec<ObjectStatus<'a>>,
    /// The postal info for the contact
    #[serde(rename = "postalInfo")]
    pub postal_info: PostalInfo<'a>,
    /// The voice data for the contact
    pub voice: Phone<'a>,
    /// The fax data for the contact
    pub fax: Option<Phone<'a>>,
    /// The email for the contact
    pub email: StringValue<'a>,
    /// The epp user to whom the contact belongs
    #[serde(rename = "clID")]
    pub client_id: StringValue<'a>,
    /// The epp user who created the contact
    #[serde(rename = "crID")]
    pub creator_id: StringValue<'a>,
    /// The creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue<'a>,
    /// The epp user who last updated the contact
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue<'a>>,
    /// The last update date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue<'a>>,
    /// The contact transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue<'a>>,
    /// The contact auth info
    #[serde(rename = "authInfo")]
    pub auth_info: Option<ContactAuthInfo<'a>>,
}

/// Type that represents the &lt;resData&gt; tag for contact info response
#[derive(Deserialize, Debug)]
pub struct ContactInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: ContactInfoData<'static>,
}

#[cfg(test)]
mod tests {
    use super::ContactInfo;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/contact/info.xml").unwrap();

        let object = ContactInfo::new("eppdev-contact-3", "eppdev-387323");

        let serialized =
            <ContactInfo as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/contact/info.xml").unwrap();
        let object =
            <ContactInfo as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

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
