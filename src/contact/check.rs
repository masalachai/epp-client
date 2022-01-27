use std::fmt::Debug;

/// Types for EPP contact check request
use super::XMLNS;
use crate::common::{CheckResponse, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for ContactCheck<'a> {}

impl<'a> Command for ContactCheck<'a> {
    type Response = CheckResponse;
    const COMMAND: &'static str = "check";
}

// Request

/// Type that represents the &lt;check&gt; command for contact transactions
#[derive(Serialize, Debug)]
pub struct ContactList<'a> {
    /// The XML namespace for the contact &lt;check&gt;
    #[serde(rename = "xmlns:contact")]
    xmlns: &'a str,
    /// The list of contact ids to check for availability
    #[serde(rename = "contact:id")]
    pub contact_ids: Vec<StringValue<'a>>,
}

#[derive(Serialize, Debug)]
/// The &lt;command&gt; type for contact check command
pub struct ContactCheck<'a> {
    /// The &lt;check&gt; tag for the contact check command
    #[serde(rename = "contact:check")]
    list: ContactList<'a>,
}

impl<'a> ContactCheck<'a> {
    pub fn new(contact_ids: &'a [&'a str]) -> Self {
        Self {
            list: ContactList {
                xmlns: XMLNS,
                contact_ids: contact_ids.iter().map(|&id| id.into()).collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContactCheck;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::response::ResultCode;
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

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(
            results.check_data.list[0].resource.id,
            "eppdev-contact-1".into()
        );
        assert!(!results.check_data.list[0].resource.available);
        assert_eq!(
            results.check_data.list[1].resource.id,
            "eppdev-contact-2".into()
        );
        assert!(results.check_data.list[1].resource.available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
