//! Types for EPP host create request

use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::XMLNS;
use crate::common::{serialize_host_addrs_option, NoExtension, StringValue};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for HostCreate<'a> {}

impl<'a> Command for HostCreate<'a> {
    type Response = HostCreateResponse;
    const COMMAND: &'static str = "create";
}

impl<'a> HostCreate<'a> {
    pub fn new(host: &'a str, addresses: Option<&'a [IpAddr]>) -> Self {
        Self {
            host: HostCreateRequestData {
                xmlns: XMLNS,
                name: host.into(),
                addresses,
            },
        }
    }
}

// Request

/// Type for data under the host &lt;create&gt; tag
#[derive(Serialize, Debug)]
pub struct HostCreateRequestData<'a> {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host")]
    xmlns: &'a str,
    /// The name of the host to be created
    #[serde(rename = "host:name")]
    pub name: StringValue<'a>,
    /// The list of IP addresses for the host
    #[serde(rename = "host:addr", serialize_with = "serialize_host_addrs_option")]
    pub addresses: Option<&'a [IpAddr]>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;create&gt; command for hosts
pub struct HostCreate<'a> {
    /// The instance holding the data for the host to be created
    #[serde(rename = "host:create")]
    host: HostCreateRequestData<'a>,
}

// Response

/// Type that represents the &lt;creData&gt; tag for host create response
#[derive(Deserialize, Debug)]
pub struct HostCreateData {
    /// The host name
    pub name: StringValue<'static>,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: DateTime<Utc>,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct HostCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: HostCreateData,
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
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.create_data.name, "host2.eppdev-1.com".into());
        assert_eq!(
            result.create_data.created_at,
            Utc.ymd(2021, 7, 26).and_hms(5, 28, 55)
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
