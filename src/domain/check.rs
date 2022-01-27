//! Types for EPP domain check request

use super::XMLNS;
use crate::common::{CheckResponse, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for DomainCheck<'a> {}

impl<'a> Command for DomainCheck<'a> {
    type Response = CheckResponse;
    const COMMAND: &'static str = "check";
}

// Request

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Debug)]
struct DomainList<'a> {
    #[serde(rename = "xmlns:domain")]
    /// XML namespace for domain commands
    xmlns: &'a str,
    #[serde(rename = "domain:name")]
    /// List of domains to be checked for availability
    domains: Vec<StringValue<'a>>,
}

#[derive(Serialize, Debug)]
struct SerializeDomainCheck<'a> {
    #[serde(rename = "domain:check")]
    list: DomainList<'a>,
}

impl<'a> From<DomainCheck<'a>> for SerializeDomainCheck<'a> {
    fn from(check: DomainCheck<'a>) -> Self {
        Self {
            list: DomainList {
                xmlns: XMLNS,
                domains: check.domains.iter().map(|&d| d.into()).collect(),
            },
        }
    }
}

/// The EPP `check` command for domains
#[derive(Clone, Debug, Serialize)]
#[serde(into = "SerializeDomainCheck")]
pub struct DomainCheck<'a> {
    /// The list of domains to be checked
    pub domains: &'a [&'a str],
}

#[cfg(test)]
mod tests {
    use super::DomainCheck;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::response::ResultCode;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/domain/check.xml").unwrap();

        let object = DomainCheck {
            domains: &["eppdev.com", "eppdev.net"],
        };

        let serialized =
            <DomainCheck as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/check.xml").unwrap();
        let object =
            <DomainCheck as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.list[0].id, "eppdev.com");
        assert!(result.list[0].available);
        assert_eq!(result.list[1].id, "eppdev.net");
        assert!(!result.list[1].available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
