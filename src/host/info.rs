//! Types for EPP host info request

use std::net::IpAddr;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use instant_xml::{FromXml, ToXml};

use super::{HostAddr, Status, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for HostInfo<'a> {}

impl<'a> Command for HostInfo<'a> {
    type Response = HostInfoResponseData;
    const COMMAND: &'static str = "info";
}

impl<'a> HostInfo<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            info: HostInfoRequestData { name },
        }
    }
}

// Request

/// Type for data under the host &lt;info&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "info", ns(XMLNS))]
pub struct HostInfoRequestData<'a> {
    /// The name of the host to be queried
    name: &'a str,
}

/// Type for EPP XML &lt;info&gt; command for hosts
#[derive(Debug, ToXml)]
#[xml(rename = "info", ns(EPP_XMLNS))]
pub struct HostInfo<'a> {
    /// The instance holding the data for the host query
    #[xml(rename = "host:info")]
    info: HostInfoRequestData<'a>,
}

// Response

/// Type that represents the &lt;infData&gt; tag for host info response
#[derive(Debug, FromXml)]
#[xml(rename = "infData", ns(XMLNS))]
pub struct HostInfoResponseData {
    /// The host name
    pub name: String,
    /// The host ROID
    pub roid: String,
    /// The list of host statuses
    #[xml(rename = "status")]
    pub statuses: Vec<Status<'static>>,
    /// The list of host IP addresses
    #[xml(rename = "addr", deserialize_with = "deserialize_host_addrs")]
    pub addresses: Vec<IpAddr>,
    /// The epp user to whom the host belongs
    #[xml(rename = "clID")]
    pub client_id: String,
    /// THe epp user that created the host
    #[xml(rename = "crID")]
    pub creator_id: String,
    /// The host creation date
    #[xml(rename = "crDate")]
    pub created_at: DateTime<Utc>,
    /// The epp user that last updated the host
    #[xml(rename = "upID")]
    pub updater_id: Option<String>,
    /// The host last update date
    #[xml(rename = "upDate")]
    pub updated_at: Option<DateTime<Utc>>,
    /// The host transfer date
    #[xml(rename = "trDate")]
    pub transferred_at: Option<DateTime<Utc>>,
}

fn deserialize_host_addrs(
    into: &mut Vec<IpAddr>,
    field: &'static str,
    deserializer: &mut instant_xml::de::Deserializer<'_, '_>,
) -> Result<(), instant_xml::Error> {
    let mut addrs = Vec::new();
    Vec::<HostAddr<'static>>::deserialize(&mut addrs, field, deserializer)?;

    for addr in addrs {
        match IpAddr::from_str(&addr.address) {
            Ok(ip) => into.push(ip),
            Err(_) => {
                return Err(instant_xml::Error::UnexpectedValue(format!(
                    "invalid IP address '{}'",
                    &addr.address
                )))
            }
        }
    }

    Ok(())
}

/*
/// Type that represents the &lt;resData&gt; tag for host info response
#[derive(Debug, FromXml)]
#[xml(rename = "infData", ns(XMLNS))]
pub struct HostInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[xml(rename = "infData")]
    pub info_data: HostInfoResponseData,
}
*/

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::{HostInfo, IpAddr};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = HostInfo::new("ns1.eppdev-1.com");
        assert_serialized("request/host/info.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostInfo>("response/host/info.xml");
        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.name, "host2.eppdev-1.com");
        assert_eq!(result.roid, "UNDEF-ROID");
        assert_eq!(result.statuses[0].status, "ok".to_string());
        assert_eq!(result.addresses[0], IpAddr::from([29, 245, 122, 14]));
        assert_eq!(
            result.addresses[1],
            IpAddr::from([0x2404, 0x6800, 0x4001, 0x801, 0, 0, 0, 0x200e])
        );
        assert_eq!(result.client_id, "eppdev");
        assert_eq!(result.creator_id, "creator");
        assert_eq!(
            result.created_at,
            Utc.with_ymd_and_hms(2021, 7, 26, 5, 28, 55).unwrap()
        );
        assert_eq!(*(result.updater_id.as_ref().unwrap()), "creator");
        assert_eq!(
            result.updated_at,
            Utc.with_ymd_and_hms(2021, 7, 26, 5, 28, 55).single()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
