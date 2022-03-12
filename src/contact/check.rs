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
struct ContactList<'a> {
    /// The XML namespace for the contact &lt;check&gt;
    #[serde(rename = "xmlns:contact")]
    xmlns: &'a str,
    /// The list of contact ids to check for availability
    #[serde(rename = "contact:id")]
    contact_ids: Vec<StringValue<'a>>,
}

#[derive(Serialize, Debug)]
struct SerializeContactCheck<'a> {
    /// The &lt;check&gt; tag for the contact check command
    #[serde(rename = "contact:check")]
    list: ContactList<'a>,
}

impl<'a> From<ContactCheck<'a>> for SerializeContactCheck<'a> {
    fn from(check: ContactCheck<'a>) -> Self {
        Self {
            list: ContactList {
                xmlns: XMLNS,
                contact_ids: check.contact_ids.iter().map(|&id| id.into()).collect(),
            },
        }
    }
}

/// The EPP `check` command for contacts
#[derive(Clone, Debug, Serialize)]
#[serde(into = "SerializeContactCheck")]
pub struct ContactCheck<'a> {
    /// The list of contact IDs to be checked
    pub contact_ids: &'a [&'a str],
}

#[cfg(test)]
mod tests {
    use super::ContactCheck;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = ContactCheck {
            contact_ids: &["eppdev-contact-1", "eppdev-contact-2"],
        };
        assert_serialized("request/contact/check.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<ContactCheck>("response/contact/check.xml");
        let results = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(results.list[0].id, "eppdev-contact-1");
        assert!(!results.list[0].available);
        assert_eq!(results.list[1].id, "eppdev-contact-2");
        assert!(results.list[1].available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
