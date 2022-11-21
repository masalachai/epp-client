//! Types for EPP host update request

use std::net::IpAddr;

use instant_xml::ToXml;

use super::{serialize_host_addrs_option, Status, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for HostUpdate<'a> {}

impl<'a> Command for HostUpdate<'a> {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl<'a> HostUpdate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            host: HostUpdateRequest {
                name,
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
    pub fn add(&mut self, add: HostAdd<'a>) {
        self.host.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; element of the host update
    pub fn remove(&mut self, remove: HostRemove<'a>) {
        self.host.remove = Some(remove);
    }
}

/// Type for data under the &lt;chg&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "chg", ns(XMLNS))]
pub struct HostChangeInfo<'a> {
    /// The new name for the host
    pub name: &'a str,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Debug, ToXml)]
#[xml(rename = "add", ns(XMLNS))]
pub struct HostAdd<'a> {
    /// The IP addresses to be added to or removed from the host
    #[xml(rename = "host:addr", serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<&'a [IpAddr]>,
    /// The statuses to be added to or removed from the host
    #[xml(rename = "host:status")]
    pub statuses: Option<&'a [Status<'a>]>,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Debug, ToXml)]
#[xml(rename = "rem", ns(XMLNS))]
pub struct HostRemove<'a> {
    /// The IP addresses to be added to or removed from the host
    #[xml(rename = "host:addr", serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<&'a [IpAddr]>,
    /// The statuses to be added to or removed from the host
    #[xml(rename = "host:status")]
    pub statuses: Option<&'a [Status<'a>]>,
}

/// Type for data under the host &lt;update&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(XMLNS))]
pub struct HostUpdateRequest<'a> {
    /// The name of the host
    name: &'a str,
    /// The IP addresses and statuses to be added to the host
    #[xml(rename = "host:add")]
    add: Option<HostAdd<'a>>,
    /// The IP addresses and statuses to be removed from the host
    #[xml(rename = "host:rem")]
    remove: Option<HostRemove<'a>>,
    /// The host details that need to be updated
    #[xml(rename = "host:chg")]
    change_info: Option<HostChangeInfo<'a>>,
}

/// Type for EPP XML &lt;update&gt; command for hosts
#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(EPP_XMLNS))]
pub struct HostUpdate<'a> {
    /// The instance holding the data for the host to be updated
    host: HostUpdateRequest<'a>,
}

#[cfg(test)]
mod tests {
    use super::IpAddr;
    use super::{HostAdd, HostChangeInfo, HostRemove, HostUpdate, Status};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let addr = &[IpAddr::from([
            0x2404, 0x6800, 0x4001, 0x801, 0, 0, 0, 0x200e,
        ])];

        let add = HostAdd {
            addresses: Some(addr),
            statuses: None,
        };

        let statuses = &[Status {
            status: "clientDeleteProhibited".into(),
        }];

        let remove = HostRemove {
            addresses: None,
            statuses: Some(statuses),
        };

        let mut object = HostUpdate::new("host1.eppdev-1.com");

        object.add(add);
        object.remove(remove);
        object.info(HostChangeInfo {
            name: "host2.eppdev-1.com",
        });

        assert_serialized("request/host/update.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostUpdate>("response/host/update.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
