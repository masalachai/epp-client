//! Types for EPP domain check request

use std::fmt;

use instant_xml::{FromXml, Serializer, ToXml};

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for DomainCheck<'a> {}

impl<'a> Command for DomainCheck<'a> {
    type Response = CheckData;
    const COMMAND: &'static str = "check";
}

// Request

#[derive(Debug, ToXml)]
#[xml(rename = "check", ns(XMLNS))]
struct DomainList<'a> {
    #[xml(rename = "name", ns(XMLNS))]
    domains: &'a [&'a str],
}

fn serialize_domains<W: fmt::Write + ?Sized>(
    domains: &[&str],
    serializer: &mut Serializer<W>,
) -> Result<(), instant_xml::Error> {
    DomainList { domains }.serialize(None, serializer)
}

#[derive(ToXml, Debug)]
#[xml(rename = "check", ns(EPP_XMLNS))]
pub struct DomainCheck<'a> {
    /// The list of domains to be checked for availability
    #[xml(serialize_with = "serialize_domains")]
    pub domains: &'a [&'a str],
}

// Response

#[derive(Debug, FromXml)]
#[xml(rename = "name", ns(XMLNS))]
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
pub struct CheckedDomain {
    /// Data under the &lt;cd&gt; tag
    #[xml(rename = "cd")]
    pub inner: Checked,
}

/// Type that represents the &lt;chkData&gt; tag for host check response
#[derive(Debug, FromXml)]
#[xml(rename = "chkData", ns(XMLNS))]
pub struct CheckData {
    pub list: Vec<CheckedDomain>,
}

#[cfg(test)]
mod tests {
    use super::DomainCheck;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = DomainCheck {
            domains: &["eppdev.com", "eppdev.net"],
        };
        assert_serialized("request/domain/check.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<DomainCheck>("response/domain/check.xml");
        let result = dbg!(&object).res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.list[0].inner.id, "eppdev.com");
        assert!(result.list[0].inner.available);
        assert_eq!(result.list[1].inner.id, "eppdev.net");
        assert!(!result.list[1].inner.available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
