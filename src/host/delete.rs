//! Types for EPP host delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for HostDelete<'a> {}

impl<'a> Command for HostDelete<'a> {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

impl<'a> HostDelete<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            host: HostDeleteRequestData {
                xmlns: XMLNS,
                name: name.into(),
            },
        }
    }
}

/// Type for data under the host &lt;delete&gt; tag
#[derive(Serialize, Debug)]
pub struct HostDeleteRequestData<'a> {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host")]
    xmlns: &'a str,
    /// The host to be deleted
    #[serde(rename = "host:name")]
    name: StringValue<'a>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;delete&gt; command for hosts
pub struct HostDelete<'a> {
    /// The instance holding the data for the host to be deleted
    #[serde(rename = "host:delete")]
    host: HostDeleteRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::HostDelete;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/delete.xml").unwrap();

        let object = HostDelete::new("ns1.eppdev-1.com");

        let serialized =
            <HostDelete as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/delete.xml").unwrap();
        let object =
            <HostDelete as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
