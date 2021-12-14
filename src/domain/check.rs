//! Types for EPP domain check request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainCheck {}

impl Command for DomainCheck {
    type Response = DomainCheckResponse;
    const COMMAND: &'static str = "check";
}

impl DomainCheck {
    pub fn new(domains: Vec<&str>) -> Self {
        Self {
            list: DomainList {
                xmlns: XMLNS,
                domains: domains
                    .into_iter()
                    .map(|d| d.into())
                    .collect::<Vec<StringValue>>(),
            },
        }
    }
}

// Request

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainList {
    #[serde(rename = "xmlns:domain")]
    /// XML namespace for domain commands
    pub xmlns: &'static str,
    #[serde(rename = "domain:name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheck {
    /// The object holding the list of domains to be checked
    #[serde(rename = "domain:check")]
    list: DomainList,
}

// Response

/// Type that represents the &lt;name&gt; tag for domain check response
#[derive(Deserialize, Debug)]
pub struct DomainAvailable {
    /// The domain name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The domain (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for domain check response
#[derive(Deserialize, Debug)]
pub struct DomainCheckResponseDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub domain: DomainAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for domain check response
#[derive(Deserialize, Debug)]
pub struct DomainCheckResponseData {
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for domain check response
#[derive(Deserialize, Debug)]
pub struct DomainCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckResponseData,
}

#[cfg(test)]
mod tests {
    use super::DomainCheck;
    use crate::common::NoExtension;
    use crate::request::Transaction;
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

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(
            result.check_data.domain_list[0].domain.name,
            "eppdev.com".into()
        );
        assert_eq!(result.check_data.domain_list[0].domain.available, 1);
        assert_eq!(
            result.check_data.domain_list[1].domain.name,
            "eppdev.net".into()
        );
        assert_eq!(result.check_data.domain_list[1].domain.available, 0);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
