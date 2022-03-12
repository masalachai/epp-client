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
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
