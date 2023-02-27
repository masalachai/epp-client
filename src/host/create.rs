//! Types for EPP host create request

use std::net::IpAddr;

use chrono::{DateTime, Utc};
use instant_xml::{FromXml, ToXml};

use super::{serialize_host_addrs_option, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for HostCreate<'a> {}

impl<'a> Command for HostCreate<'a> {
    type Response = CreateData;
    const COMMAND: &'static str = "create";
}

impl<'a> HostCreate<'a> {
    pub fn new(name: &'a str, addresses: Option<&'a [IpAddr]>) -> Self {
        Self {
            host: HostCreateRequest { name, addresses },
        }
    }
}

// Request

/// Type for data under the host &lt;create&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "create", ns(XMLNS))]
pub struct HostCreateRequest<'a> {
    /// The name of the host to be created
    pub name: &'a str,
    /// The list of IP addresses for the host
    #[xml(serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<&'a [IpAddr]>,
}

/// Type for EPP XML &lt;create&gt; command for hosts
#[derive(Debug, ToXml)]
#[xml(rename = "create", ns(EPP_XMLNS))]
pub struct HostCreate<'a> {
    /// The instance holding the data for the host to be created
    host: HostCreateRequest<'a>,
}

// Response

/// Type that represents the &lt;creData&gt; tag for host create response
#[derive(Debug, FromXml)]
#[xml(rename = "creData", ns(XMLNS))]
pub struct CreateData {
    /// The host name
    pub name: String,
    /// The host creation date
    #[xml(rename = "crDate")]
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::{HostCreate, IpAddr};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let addresses = &[
            IpAddr::from([29, 245, 122, 14]),
            IpAddr::from([0x2404, 0x6800, 0x4001, 0x801, 0, 0, 0, 0x200e]),
        ];

        let object = HostCreate::new("host1.eppdev-1.com", Some(addresses));
        assert_serialized("request/host/create.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostCreate>("response/host/create.xml");
        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.name, "host2.eppdev-1.com");
        assert_eq!(
            result.created_at,
            Utc.with_ymd_and_hms(2021, 7, 26, 5, 28, 55).unwrap()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
