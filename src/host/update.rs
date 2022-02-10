//! Types for EPP host update request

use std::net::IpAddr;

use super::XMLNS;
use crate::common::{serialize_host_addrs_option, NoExtension, ObjectStatus, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl<'a> Transaction<NoExtension> for HostUpdate<'a> {}

impl<'a> Command for HostUpdate<'a> {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl<'a> HostUpdate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            host: HostUpdateRequestData {
                xmlns: XMLNS,
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; element of the host update
    pub fn info(&mut self, info: HostChangeInfo<'a>) {
        self.host.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; element of the host update
    pub fn add(&mut self, add: HostAddRemove<'a>) {
        self.host.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; element of the host update
    pub fn remove(&mut self, remove: HostAddRemove<'a>) {
        self.host.remove = Some(remove);
    }
}

/// Type for data under the &lt;chg&gt; tag
#[derive(Serialize, Debug)]
pub struct HostChangeInfo<'a> {
    /// The new name for the host
    #[serde(rename = "host:name")]
    pub name: StringValue<'a>,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Serialize, Debug)]
pub struct HostAddRemove<'a> {
    /// The IP addresses to be added to or removed from the host
    #[serde(rename = "host:addr", serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<&'a [IpAddr]>,
    /// The statuses to be added to or removed from the host
    #[serde(rename = "host:status")]
    pub statuses: Option<&'a [ObjectStatus<'a>]>,
}

/// Type for data under the host &lt;update&gt; tag
#[derive(Serialize, Debug)]
pub struct HostUpdateRequestData<'a> {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host")]
    xmlns: &'a str,
    /// The name of the host
    #[serde(rename = "host:name")]
    name: StringValue<'a>,
    /// The IP addresses and statuses to be added to the host
    #[serde(rename = "host:add")]
    add: Option<HostAddRemove<'a>>,
    /// The IP addresses and statuses to be removed from the host
    #[serde(rename = "host:rem")]
    remove: Option<HostAddRemove<'a>>,
    /// The host details that need to be updated
    #[serde(rename = "host:chg")]
    change_info: Option<HostChangeInfo<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;update&gt; command for hosts
pub struct HostUpdate<'a> {
    /// The instance holding the data for the host to be updated
    #[serde(rename = "host:update")]
    host: HostUpdateRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::IpAddr;
    use super::{HostAddRemove, HostChangeInfo, HostUpdate};
    use crate::common::{NoExtension, ObjectStatus};
    use crate::request::Transaction;
    use crate::response::ResultCode;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/update.xml").unwrap();

        let addr = &[IpAddr::from([
            0x2404, 0x6800, 0x4001, 0x801, 0, 0, 0, 0x200e,
        ])];

        let add = HostAddRemove {
            addresses: Some(addr),
            statuses: None,
        };

        let statuses = &[ObjectStatus {
            status: "clientDeleteProhibited".into(),
        }];

        let remove = HostAddRemove {
            addresses: None,
            statuses: Some(statuses),
        };

        let mut object = HostUpdate::new("host1.eppdev-1.com");

        object.add(add);
        object.remove(remove);
        object.info(HostChangeInfo {
            name: "host2.eppdev-1.com".into(),
        });

        let serialized =
            <HostUpdate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/update.xml").unwrap();
        let object =
            <HostUpdate as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
