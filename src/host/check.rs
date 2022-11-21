//! Types for EPP host check request

use std::fmt::{self, Debug};

use instant_xml::{FromXml, Serializer, ToXml};

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for HostCheck<'a> {}

impl<'a> Command for HostCheck<'a> {
    type Response = CheckData;
    const COMMAND: &'static str = "check";
}

// Request

/// Type for data under the host &lt;check&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "check", ns(XMLNS))]
struct HostCheckData<'a> {
    /// List of hosts to be checked for availability
    name: &'a [&'a str],
}

fn serialize_hosts<W: fmt::Write + ?Sized>(
    hosts: &[&str],
    serializer: &mut Serializer<W>,
) -> Result<(), instant_xml::Error> {
    HostCheckData { name: hosts }.serialize(None, serializer)
}

/// The EPP `check` command for hosts
#[derive(Clone, Debug, ToXml)]
#[xml(rename = "check", ns(EPP_XMLNS))]
pub struct HostCheck<'a> {
    /// The list of hosts to be checked
    #[xml(serialize_with = "serialize_hosts")]
    pub hosts: &'a [&'a str],
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
pub struct CheckedHost {
    /// Data under the &lt;cd&gt; tag
    #[xml(rename = "cd")]
    pub inner: Checked,
}

/// Type that represents the &lt;chkData&gt; tag for host check response
#[derive(Debug, FromXml)]
#[xml(rename = "chkData", ns(XMLNS))]
pub struct CheckData {
    pub list: Vec<CheckedHost>,
}

#[cfg(test)]
mod tests {
    use super::HostCheck;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = HostCheck {
            hosts: &["ns1.eppdev-1.com", "host1.eppdev-1.com"],
        };
        assert_serialized("request/host/check.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostCheck>("response/host/check.xml");
        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.list[0].inner.id, "host1.eppdev-1.com");
        assert!(result.list[0].inner.available);
        assert_eq!(result.list[1].inner.id, "ns1.testing.com");
        assert!(!result.list[1].inner.available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
