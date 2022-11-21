//! Types for EPP contact check request

use std::fmt::{self, Debug};

use instant_xml::{FromXml, Serializer, ToXml};

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for ContactCheck<'a> {}

impl<'a> Command for ContactCheck<'a> {
    type Response = CheckData;
    const COMMAND: &'static str = "check";
}

// Request

/// Type that represents the &lt;check&gt; command for contact transactions
#[derive(Debug, ToXml)]
#[xml(rename = "check", ns(XMLNS))]
struct ContactList<'a> {
    /// The list of contact ids to check for availability
    id: &'a [&'a str],
}

fn serialize_contacts<W: fmt::Write + ?Sized>(
    ids: &[&str],
    serializer: &mut Serializer<W>,
) -> Result<(), instant_xml::Error> {
    ContactList { id: ids }.serialize(None, serializer)
}

/// The EPP `check` command for contacts
#[derive(Clone, Debug, ToXml)]
#[xml(rename = "check", ns(EPP_XMLNS))]
pub struct ContactCheck<'a> {
    #[xml(serialize_with = "serialize_contacts")]
    pub contact_ids: &'a [&'a str],
}

// Response

#[derive(Debug, FromXml)]
#[xml(rename = "id", ns(XMLNS))]
pub struct Checked {
    #[xml(attribute, rename = "avail")]
    pub available: bool,
    #[xml(attribute)]
    pub reason: Option<String>,
    #[xml(direct)]
    pub id: String,
}

#[derive(Debug, FromXml)]
#[xml(rename = "cd", ns(XMLNS))]
pub struct CheckedContact {
    /// Data under the &lt;cd&gt; tag
    pub inner: Checked,
}

/// Type that represents the &lt;chkData&gt; tag for host check response
#[derive(Debug, FromXml)]
#[xml(rename = "chkData", ns(XMLNS))]
pub struct CheckData {
    pub list: Vec<CheckedContact>,
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
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(results.list[0].inner.id, "eppdev-contact-1");
        assert!(!results.list[0].inner.available);
        assert_eq!(results.list[1].inner.id, "eppdev-contact-2");
        assert!(results.list[1].inner.available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
