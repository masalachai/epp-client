//! Types for EPP contact create request

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ContactAuthInfo, Phone, PostalInfo, XMLNS};
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for ContactCreate<'a> {}

impl<'a> Command for ContactCreate<'a> {
    type Response = ContactCreateResponse;
    const COMMAND: &'static str = "create";
}

// Request

/// Type for elements under the contact &lt;create&gt; tag
#[derive(Serialize, Debug)]
pub struct Contact<'a> {
    /// XML namespace for contact commands
    #[serde(rename = "xmlns:contact")]
    xmlns: &'a str,
    /// Contact &lt;id&gt; tag
    #[serde(rename = "contact:id")]
    id: StringValue<'a>,
    /// Contact &lt;postalInfo&gt; tag
    #[serde(rename = "contact:postalInfo")]
    postal_info: PostalInfo<'a>,
    /// Contact &lt;voice&gt; tag
    #[serde(rename = "contact:voice")]
    voice: Phone<'a>,
    /// Contact &lt;fax&gt; tag,
    #[serde(rename = "contact:fax")]
    fax: Option<Phone<'a>>,
    /// Contact &lt;email&gt; tag
    #[serde(rename = "contact:email")]
    email: StringValue<'a>,
    /// Contact &lt;authInfo&gt; tag
    #[serde(rename = "contact:authInfo")]
    auth_info: ContactAuthInfo<'a>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;create&gt; command for contacts
pub struct ContactCreate<'a> {
    /// Data for &lt;create&gt; command for contact
    #[serde(rename = "contact:create")]
    pub contact: Contact<'a>,
}

impl<'a> ContactCreate<'a> {
    pub fn new(
        id: &'a str,
        email: &'a str,
        postal_info: PostalInfo<'a>,
        voice: Phone<'a>,
        auth_password: &'a str,
    ) -> Self {
        Self {
            contact: Contact {
                xmlns: XMLNS,
                id: id.into(),
                postal_info,
                voice,
                fax: None,
                email: email.into(),
                auth_info: ContactAuthInfo::new(auth_password),
            },
        }
    }

    /// Sets the &lt;fax&gt; data for the request
    pub fn set_fax(&mut self, fax: Phone<'a>) {
        self.contact.fax = Some(fax);
    }
}

// Response

/// Type that represents the &lt;creData&gt; tag for contact create response
#[derive(Deserialize, Debug)]
pub struct ContactCreateData {
    /// The contact id
    pub id: StringValue<'static>,
    #[serde(rename = "crDate")]
    /// The contact creation date
    pub created_at: DateTime<Utc>,
}

/// Type that represents the &lt;resData&gt; tag for contact create response
#[derive(Deserialize, Debug)]
pub struct ContactCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: ContactCreateData,
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::{ContactCreate, Phone, PostalInfo};
    use crate::contact::Address;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let street = &["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR".parse().unwrap());
        let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
        let mut voice = Phone::new("+33.47237942");
        voice.set_extension("123");
        let mut fax = Phone::new("+33.86698799");
        fax.set_extension("677");

        let mut object = ContactCreate::new(
            "eppdev-contact-3",
            "contact@eppdev.net",
            postal_info,
            voice,
            "eppdev-387323",
        );
        object.set_fax(fax);

        assert_serialized("request/contact/create.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<ContactCreate>("response/contact/create.xml");
        let results = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(results.create_data.id, "eppdev-contact-4".into());
        assert_eq!(
            results.create_data.created_at,
            Utc.ymd(2021, 7, 25).and_hms(16, 5, 32)
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
