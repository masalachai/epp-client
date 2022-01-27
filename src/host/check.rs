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

impl<'a> HostCheck<'a> {
    pub fn new(hosts: &[&'a str]) -> Self {
        let hosts = hosts.iter().map(|&d| d.into()).collect();

        Self {
            list: HostList {
                xmlns: XMLNS,
                hosts,
            },
        }
    }
}

// Request

/// Type for data under the host &lt;check&gt; tag
#[derive(Serialize, Debug)]
pub struct HostList<'a> {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host")]
    xmlns: &'a str,
    /// List of hosts to be checked for availability
    #[serde(rename = "host:name")]
    pub hosts: Vec<StringValue<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for hosts
pub struct HostCheck<'a> {
    /// The instance holding the list of hosts to be checked
    #[serde(rename = "host:check")]
    list: HostList<'a>,
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

        let object = HostCheck::new(&["ns1.eppdev-1.com", "host1.eppdev-1.com"]);

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
