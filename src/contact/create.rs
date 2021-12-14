//! Types for EPP contact create request

use super::{ContactAuthInfo, Phone, PostalInfo, XMLNS};
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

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
    voice: Phone,
    /// Contact &lt;fax&gt; tag,
    #[serde(rename = "contact:fax")]
    fax: Option<Phone>,
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
        voice: Phone,
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
    pub fn set_fax(&mut self, fax: Phone) {
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
    pub created_at: StringValue<'static>,
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
    use super::{ContactCreate, Phone, PostalInfo};
    use crate::common::NoExtension;
    use crate::contact::Address;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/contact/create.xml").unwrap();

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

        let serialized =
            <ContactCreate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/contact/create.xml").unwrap();
        let object =
            <ContactCreate as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        let results = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(results.create_data.id, "eppdev-contact-4".into());
        assert_eq!(
            results.create_data.created_at,
            "2021-07-25T16:05:32.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
