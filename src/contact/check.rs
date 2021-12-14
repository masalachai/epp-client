use std::fmt::Debug;

/// Types for EPP contact check request
use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for ContactCheck {}

// Request

/// Type that represents the &lt;check&gt; command for contact transactions
#[derive(Serialize, Debug)]
pub struct ContactList {
    /// The XML namespace for the contact &lt;check&gt;
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// The list of contact ids to check for availability
    #[serde(rename = "contact:id")]
    pub contact_ids: Vec<StringValue>,
}

#[derive(Serialize, Debug)]
/// The &lt;command&gt; type for contact check command
pub struct ContactCheck {
    /// The &lt;check&gt; tag for the contact check command
    #[serde(rename = "contact:check")]
    list: ContactList,
}

impl ContactCheck {
    pub fn new(contact_ids: &[&str]) -> Self {
        let contact_ids = contact_ids
            .iter()
            .map(|&d| d.into())
            .collect::<Vec<StringValue>>();

        Self {
            list: ContactList {
                xmlns: XMLNS.to_string(),
                contact_ids,
            },
        }
    }
}

impl Command for ContactCheck {
    type Response = ContactCheckResponse;
    const COMMAND: &'static str = "check";
}

// Response

/// Type that represents the &lt;id&gt; tag for contact check response
#[derive(Deserialize, Debug)]
pub struct ContactAvailable {
    /// The text of the &lt;id&gt; tag
    #[serde(rename = "$value")]
    pub id: StringValue,
    /// The avail attr on the &lt;id&gt; tag
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for contact check response
#[derive(Deserialize, Debug)]
pub struct ContactCheckResponseDataItem {
    /// Data under the &lt;id&gt; tag
    #[serde(rename = "id")]
    pub contact: ContactAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for contact check response
#[derive(Deserialize, Debug)]
pub struct ContactCheckResponseData {
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub contact_list: Vec<ContactCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for contact check response
#[derive(Deserialize, Debug)]
pub struct ContactCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: ContactCheckResponseData,
}

#[cfg(test)]
mod tests {
    use super::ContactCheck;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/contact/check.xml").unwrap();
        let object = ContactCheck::new(&["eppdev-contact-1", "eppdev-contact-2"]);
        let serialized =
            <ContactCheck as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/contact/check.xml").unwrap();
        let object =
            <ContactCheck as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let results = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(
            results.check_data.contact_list[0].contact.id,
            "eppdev-contact-1".into()
        );
        assert_eq!(results.check_data.contact_list[0].contact.available, 0);
        assert_eq!(
            results.check_data.contact_list[1].contact.id,
            "eppdev-contact-2".into()
        );
        assert_eq!(results.check_data.contact_list[1].contact.available, 1);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
