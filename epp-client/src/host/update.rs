//! Types for EPP host update request

use super::XMLNS;
use crate::common::{HostAddr, HostStatus, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl Transaction<NoExtension> for HostUpdate {}

impl Command for HostUpdate {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl HostUpdate {
    pub fn new(name: &str) -> Self {
        Self {
            host: HostUpdateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; element of the host update
    pub fn info(&mut self, info: HostChangeInfo) {
        self.host.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; element of the host update
    pub fn add(&mut self, add: HostAddRemove) {
        self.host.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; element of the host update
    pub fn remove(&mut self, remove: HostAddRemove) {
        self.host.remove = Some(remove);
    }
}

/// Type for data under the &lt;chg&gt; tag
#[derive(Serialize, Debug)]
pub struct HostChangeInfo {
    /// The new name for the host
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Serialize, Debug)]
pub struct HostAddRemove {
    /// The IP addresses to be added to or removed from the host
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
    /// The statuses to be added to or removed from the host
    #[serde(rename = "host:status", alias = "status")]
    pub statuses: Option<Vec<HostStatus>>,
}

/// Type for data under the host &lt;update&gt; tag
#[derive(Serialize, Debug)]
pub struct HostUpdateRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
    /// The IP addresses and statuses to be added to the host
    #[serde(rename = "host:add", alias = "add")]
    add: Option<HostAddRemove>,
    /// The IP addresses and statuses to be removed from the host
    #[serde(rename = "host:rem", alias = "rem")]
    remove: Option<HostAddRemove>,
    /// The host details that need to be updated
    #[serde(rename = "host:chg", alias = "chg")]
    change_info: Option<HostChangeInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;update&gt; command for hosts
pub struct HostUpdate {
    /// The instance holding the data for the host to be updated
    #[serde(rename = "host:update", alias = "update")]
    host: HostUpdateRequestData,
}

#[cfg(test)]
mod tests {
    use super::{HostAddRemove, HostChangeInfo, HostUpdate};
    use crate::common::{HostAddr, HostStatus};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/update.xml").unwrap();

        let addr = vec![HostAddr::new("v6", "2404:6800:4001:801::200e")];

        let add = HostAddRemove {
            addresses: Some(addr),
            statuses: None,
        };

        let remove = HostAddRemove {
            addresses: None,
            statuses: Some(vec![HostStatus {
                status: "clientDeleteProhibited".to_string(),
            }]),
        };

        let mut object = HostUpdate::new("host1.eppdev-1.com");

        object.add(add);
        object.remove(remove);
        object.info(HostChangeInfo {
            name: "host2.eppdev-1.com".into(),
        });

        let serialized = object.serialize_request(None, CLTRID).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/update.xml").unwrap();
        let object = HostUpdate::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
