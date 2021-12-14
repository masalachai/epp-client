//! Types for EPP contact create request

use super::{ContactAuthInfo, Phone, PostalInfo, XMLNS};
use crate::common::{NoExtension, ObjectStatus, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for ContactUpdate<'a> {}

impl<'a> Command for ContactUpdate<'a> {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl<'a> ContactUpdate<'a> {
    pub fn new(id: &'a str) -> ContactUpdate {
        Self {
            contact: ContactUpdateRequestData {
                xmlns: XMLNS,
                id: id.into(),
                add_statuses: None,
                remove_statuses: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; tag for the contact update request
    pub fn set_info(
        &mut self,
        email: &'a str,
        postal_info: PostalInfo<'a>,
        voice: Phone,
        auth_password: &'a str,
    ) {
        self.contact.change_info = Some(ContactChangeInfo {
            email: Some(email.into()),
            postal_info: Some(postal_info),
            voice: Some(voice),
            auth_info: Some(ContactAuthInfo::new(auth_password)),
            fax: None,
        });
    }

    /// Sets the data for the &lt;fax&gt; tag under &lt;chg&gt; for the contact update request
    pub fn set_fax(&mut self, fax: Phone) {
        if let Some(info) = &mut self.contact.change_info {
            info.fax = Some(fax)
        }
    }

    /// Sets the data for the &lt;add&gt; tag for the contact update request
    pub fn add(&mut self, status: &'a [ObjectStatus]) {
        self.contact.add_statuses = Some(StatusList { status });
    }

    /// Sets the data for the &lt;rem&gt; tag for the contact update request
    pub fn remove(&mut self, status: &'a [ObjectStatus]) {
        self.contact.remove_statuses = Some(StatusList { status });
    }
}

/// Type for elements under the &lt;chg&gt; tag for contact update request
#[derive(Serialize, Debug)]
pub struct ContactChangeInfo<'a> {
    #[serde(rename = "contact:postalInfo")]
    postal_info: Option<PostalInfo<'a>>,
    #[serde(rename = "contact:voice")]
    voice: Option<Phone>,
    #[serde(rename = "contact:fax")]
    fax: Option<Phone>,
    #[serde(rename = "contact:email")]
    email: Option<StringValue<'a>>,
    #[serde(rename = "contact:authInfo")]
    auth_info: Option<ContactAuthInfo<'a>>,
}

/// Type for list of elements of the &lt;status&gt; tag for contact update request
#[derive(Serialize, Debug)]
pub struct StatusList<'a> {
    #[serde(rename = "contact:status")]
    status: &'a [ObjectStatus],
}

/// Type for elements under the contact &lt;update&gt; tag
#[derive(Serialize, Debug)]
pub struct ContactUpdateRequestData<'a> {
    #[serde(rename = "xmlns:contact")]
    xmlns: &'a str,
    #[serde(rename = "contact:id")]
    id: StringValue<'a>,
    #[serde(rename = "contact:add")]
    add_statuses: Option<StatusList<'a>>,
    #[serde(rename = "contact:rem")]
    remove_statuses: Option<StatusList<'a>>,
    #[serde(rename = "contact:chg")]
    change_info: Option<ContactChangeInfo<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;update&gt; command for contacts
pub struct ContactUpdate<'a> {
    /// The data under the &lt;update&gt; tag for the contact update
    #[serde(rename = "contact:update")]
    contact: ContactUpdateRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::{ContactUpdate, Phone, PostalInfo};
    use crate::common::{NoExtension, ObjectStatus};
    use crate::contact::Address;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/contact/update.xml").unwrap();

        let mut object = ContactUpdate::new("eppdev-contact-3");

        let street = &["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR".parse().unwrap());
        let postal_info = PostalInfo::new("loc", "John Doe", "Acme Widgets", address);
        let voice = Phone::new("+33.47237942");

        object.set_info("newemail@eppdev.net", postal_info, voice, "eppdev-387323");
        let add_statuses = &[ObjectStatus {
            status: "clientTransferProhibited".to_string(),
        }];
        object.add(add_statuses);
        let remove_statuses = &[ObjectStatus {
            status: "clientDeleteProhibited".to_string(),
        }];
        object.remove(remove_statuses);

        let serialized =
            <ContactUpdate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_update() {
        let xml = get_xml("response/contact/update.xml").unwrap();
        let object =
            <ContactUpdate as Transaction<NoExtension>>::deserialize_response(xml.as_str())
                .unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
