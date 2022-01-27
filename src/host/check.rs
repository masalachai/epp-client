//! Types for EPP host check request

use std::fmt::Debug;

use super::XMLNS;
use crate::common::{CheckResponse, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for HostCheck<'a> {}

impl<'a> Command for HostCheck<'a> {
    type Response = CheckResponse;
    const COMMAND: &'static str = "check";
}

// Request

/// Type for data under the host &lt;check&gt; tag
#[derive(Serialize, Debug)]
struct HostList<'a> {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host")]
    xmlns: &'a str,
    /// List of hosts to be checked for availability
    #[serde(rename = "host:name")]
    hosts: Vec<StringValue<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for hosts
struct SerializeHostCheck<'a> {
    /// The instance holding the list of hosts to be checked
    #[serde(rename = "host:check")]
    list: HostList<'a>,
}

impl<'a> From<HostCheck<'a>> for SerializeHostCheck<'a> {
    fn from(check: HostCheck<'a>) -> Self {
        Self {
            list: HostList {
                xmlns: XMLNS,
                hosts: check.hosts.iter().map(|&id| id.into()).collect(),
            },
        }
    }
}

/// The EPP `check` command for hosts
#[derive(Clone, Debug, Serialize)]
#[serde(into = "SerializeHostCheck")]
pub struct HostCheck<'a> {
    /// The list of hosts to be checked
    pub hosts: &'a [&'a str],
}

#[cfg(test)]
mod tests {
    use super::HostCheck;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::response::ResultCode;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/check.xml").unwrap();

        let object = HostCheck {
            hosts: &["ns1.eppdev-1.com", "host1.eppdev-1.com"],
        };

        let serialized =
            <HostCheck as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/check.xml").unwrap();
        let object =
            <HostCheck as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.list[0].id, "host1.eppdev-1.com");
        assert!(result.list[0].available);
        assert_eq!(result.list[1].id, "ns1.testing.com");
        assert!(!result.list[1].available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
