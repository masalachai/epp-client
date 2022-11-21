//! Types for EPP domain delete request

use instant_xml::ToXml;

use super::XMLNS;
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for DomainDelete<'a> {}

impl<'a> Command for DomainDelete<'a> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

impl<'a> DomainDelete<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            domain: DomainDeleteRequestData { name },
        }
    }
}

/// Type for &lt;name&gt; element under the domain &lt;delete&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "delete", ns(XMLNS))]
pub struct DomainDeleteRequestData<'a> {
    /// The domain to be deleted
    name: &'a str,
}

#[derive(Debug, ToXml)]
/// Type for EPP XML &lt;delete&gt; command for domains
#[xml(rename = "delete", ns(EPP_XMLNS))]
pub struct DomainDelete<'a> {
    /// The data under the &lt;delete&gt; tag for domain deletion
    domain: DomainDeleteRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::DomainDelete;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = DomainDelete::new("eppdev.com");
        assert_serialized("request/domain/delete.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<DomainDelete>("response/domain/delete.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
