//! Types for EPP host delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl Transaction<NoExtension> for HostDelete {}

impl Command for HostDelete {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

impl HostDelete {
    pub fn new(name: &str) -> Self {
        Self {
            host: HostDeleteRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
            },
        }
    }
}

/// Type for data under the host &lt;delete&gt; tag
#[derive(Serialize, Debug)]
pub struct HostDeleteRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The host to be deleted
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;delete&gt; command for hosts
pub struct HostDelete {
    /// The instance holding the data for the host to be deleted
    #[serde(rename = "host:delete", alias = "delete")]
    host: HostDeleteRequestData,
}

#[cfg(test)]
mod tests {
    use super::HostDelete;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/delete.xml").unwrap();

        let object = HostDelete::new("ns1.eppdev-1.com");

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/delete.xml").unwrap();
        let object = HostDelete::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
