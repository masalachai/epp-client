//! Types for EPP domain delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for DomainDelete<'a> {}

impl<'a> Command for DomainDelete<'a> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

impl<'a> DomainDelete<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            domain: DomainDeleteRequestData {
                xmlns: XMLNS,
                name: name.into(),
            },
        }
    }
}

/// Type for &lt;name&gt; element under the domain &lt;delete&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainDeleteRequestData<'a> {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    xmlns: &'a str,
    /// The domain to be deleted
    #[serde(rename = "domain:name")]
    name: StringValue<'a>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;delete&gt; command for domains
pub struct DomainDelete<'a> {
    /// The data under the &lt;delete&gt; tag for domain deletion
    #[serde(rename = "domain:delete")]
    domain: DomainDeleteRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::DomainDelete;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/domain/delete.xml").unwrap();

        let object = DomainDelete::new("eppdev.com");

        let serialized =
            <DomainDelete as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/delete.xml").unwrap();
        let object =
            <DomainDelete as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
