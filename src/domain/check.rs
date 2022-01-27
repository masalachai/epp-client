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

impl<'a> DomainCheck<'a> {
    pub fn new(domains: Vec<&'a str>) -> Self {
        Self {
            list: DomainList {
                xmlns: XMLNS,
                domains: domains.into_iter().map(|d| d.into()).collect(),
            },
        }
    }
}

// Request

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainList<'a> {
    #[serde(rename = "xmlns:domain")]
    /// XML namespace for domain commands
    pub xmlns: &'a str,
    #[serde(rename = "domain:name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheck<'a> {
    /// The object holding the list of domains to be checked
    #[serde(rename = "domain:check")]
    list: DomainList<'a>,
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

        let object = DomainCheck::new(vec!["eppdev.com", "eppdev.net"]);

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
        assert_eq!(result.check_data.list[0].resource.name, "eppdev.com".into());
        assert!(result.check_data.list[0].resource.available);
        assert_eq!(result.check_data.list[1].resource.name, "eppdev.net".into());
        assert!(!result.check_data.list[1].resource.available);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
