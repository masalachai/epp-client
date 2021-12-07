//! Types for EPP contact create request

use super::XMLNS;
use crate::common::{ContactAuthInfo, NoExtension, Phone, PostalInfo, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for ContactCreate {}

impl Command for ContactCreate {
    type Response = ContactCreateResponse;
    const COMMAND: &'static str = "create";
}

// Request

/// Type for elements under the contact &lt;create&gt; tag
#[derive(Serialize, Debug)]
pub struct Contact {
    /// XML namespace for contact commands
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// Contact &lt;id&gt; tag
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
    /// Contact &lt;postalInfo&gt; tag
    #[serde(rename = "contact:postalInfo", alias = "postalInfo")]
    postal_info: PostalInfo,
    /// Contact &lt;voice&gt; tag
    #[serde(rename = "contact:voice", alias = "voice")]
    voice: Phone,
    /// Contact &lt;fax&gt; tag,
    #[serde(rename = "contact:fax", alias = "fax")]
    fax: Option<Phone>,
    /// Contact &lt;email&gt; tag
    #[serde(rename = "contact:email", alias = "email")]
    email: StringValue,
    /// Contact &lt;authInfo&gt; tag
    #[serde(rename = "contact:authInfo", alias = "authInfo")]
    auth_info: ContactAuthInfo,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;create&gt; command for contacts
pub struct ContactCreate {
    /// Data for &lt;create&gt; command for contact
    #[serde(rename = "contact:create", alias = "create")]
    pub contact: Contact,
}

impl ContactCreate {
    pub fn new(
        id: &str,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
    ) -> Self {
        Self {
            contact: Contact {
                xmlns: XMLNS.to_string(),
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
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// The contact id
    pub id: StringValue,
    #[serde(rename = "crDate")]
    /// The contact creation date
    pub created_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for contact create response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: ContactCreateData,
}

#[cfg(test)]
mod tests {
    use super::ContactCreate;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn contact_create() {
        let xml = get_xml("response/contact/create.xml").unwrap();
        let object = ContactCreate::deserialize_response(xml.as_str()).unwrap();

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
