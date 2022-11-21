//! Types for EPP contact info request

use chrono::{DateTime, Utc};
use instant_xml::{FromXml, ToXml};

use super::{ContactAuthInfo, Fax, PostalInfo, Status, Voice, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for ContactInfo<'a> {}

impl<'a> Command for ContactInfo<'a> {
    type Response = ContactInfoData;
    const COMMAND: &'static str = "info";
}

// Request

/// Type for elements under the contact &lt;info&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "info", ns(XMLNS))]
pub struct ContactInfoRequest<'a> {
    /// The contact id for the info command
    id: &'a str,
    /// The &lt;authInfo&gt; data
    auth_info: ContactAuthInfo<'a>,
}

/// Type for EPP XML &lt;info&gt; command for contacts
#[derive(Debug, ToXml)]
#[xml(rename = "info", ns(EPP_XMLNS))]
pub struct ContactInfo<'a> {
    /// Data for &lt;info&gt; command for contact
    info: ContactInfoRequest<'a>,
}

impl<'a> ContactInfo<'a> {
    pub fn new(id: &'a str, auth_password: &'a str) -> ContactInfo<'a> {
        Self {
            info: ContactInfoRequest {
                id,
                auth_info: ContactAuthInfo::new(auth_password),
            },
        }
    }
}

// Response

/// Type that represents the &lt;infData&gt; tag for contact check response
#[derive(Debug, FromXml)]
#[xml(rename = "infData", ns(XMLNS))]
pub struct ContactInfoData {
    /// The contact id
    pub id: String,
    /// The contact ROID
    pub roid: String,
    /// The list of contact statuses
    pub statuses: Vec<Status<'static>>,
    /// The postal info for the contact
    pub postal_info: PostalInfo<'static>,
    /// The voice data for the contact
    pub voice: Voice<'static>,
    /// The fax data for the contact
    pub fax: Option<Fax<'static>>,
    /// The email for the contact
    pub email: String,
    /// The epp user to whom the contact belongs
    #[xml(rename = "clID")]
    pub client_id: String,
    /// The epp user who created the contact
    #[xml(rename = "crID")]
    pub creator_id: String,
    /// The creation date
    #[xml(rename = "crDate")]
    pub created_at: DateTime<Utc>,
    /// The epp user who last updated the contact
    #[xml(rename = "upID")]
    pub updater_id: Option<String>,
    /// The last update date
    #[xml(rename = "upDate")]
    pub updated_at: Option<DateTime<Utc>>,
    /// The contact transfer date
    #[xml(rename = "trDate")]
    pub transferred_at: Option<DateTime<Utc>>,
    /// The contact auth info
    #[xml(rename = "authInfo")]
    pub auth_info: Option<ContactAuthInfo<'static>>,
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::ContactInfo;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = ContactInfo::new("eppdev-contact-3", "eppdev-387323");
        assert_serialized("request/contact/info.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<ContactInfo>("response/contact/info.xml");

        let result = object.res_data().unwrap();
        let fax = result.fax.as_ref().unwrap();
        let voice_ext = result.voice.extension.as_ref().unwrap();
        let fax_ext = fax.extension.as_ref().unwrap();
        let auth_info = result.auth_info.as_ref().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.id, "eppdev-contact-3");
        assert_eq!(result.roid, "UNDEF-ROID");
        assert_eq!(result.statuses[0].status, "ok");
        assert_eq!(result.postal_info.info_type, "loc");
        assert_eq!(result.postal_info.name, "John Doe");
        assert_eq!(result.postal_info.organization, "Acme Widgets");
        assert_eq!(result.postal_info.address.street[0], "58");
        assert_eq!(result.postal_info.address.street[1], "Orchid Road");
        assert_eq!(result.postal_info.address.city, "Paris");
        assert_eq!(result.postal_info.address.province, "Paris");
        assert_eq!(result.postal_info.address.postal_code, "392374");
        assert_eq!(result.postal_info.address.country.alpha2, "FR");
        assert_eq!(result.voice.number, "+33.47237942".to_string());
        assert_eq!(*voice_ext, "123".to_string());
        assert_eq!(fax.number, "+33.86698799".to_string());
        assert_eq!(*fax_ext, "243".to_string());
        assert_eq!(result.email, "contact@eppdev.net");
        assert_eq!(result.client_id, "eppdev");
        assert_eq!(result.creator_id, "SYSTEM");
        assert_eq!(
            result.created_at,
            Utc.with_ymd_and_hms(2021, 7, 23, 13, 9, 9).unwrap(),
        );
        assert_eq!(*(result.updater_id.as_ref().unwrap()), "SYSTEM");
        assert_eq!(
            result.updated_at,
            Utc.with_ymd_and_hms(2021, 7, 23, 13, 9, 9).single()
        );
        assert_eq!(auth_info.password, "eppdev-387323");
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
