//! Types for EPP host info request

use super::XMLNS;
use crate::common::{HostAddr, NoExtension, ObjectStatus, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for HostInfo {}

impl Command for HostInfo {
    type Response = HostInfoResponse;
    const COMMAND: &'static str = "info";
}

impl HostInfo {
    pub fn new(name: &str) -> Self {
        Self {
            info: HostInfoRequestData {
                xmlns: XMLNS,
                name: name.into(),
            },
        }
    }
}

// Request

/// Type for data under the host &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct HostInfoRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host")]
    xmlns: &'static str,
    /// The name of the host to be queried
    #[serde(rename = "host:name")]
    name: StringValue,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;info&gt; command for hosts
pub struct HostInfo {
    /// The instance holding the data for the host query
    #[serde(rename = "host:info")]
    info: HostInfoRequestData,
}

// Response

/// Type that represents the &lt;infData&gt; tag for host info response
#[derive(Deserialize, Debug)]
pub struct HostInfoResponseData {
    /// The host name
    pub name: StringValue,
    /// The host ROID
    pub roid: StringValue,
    /// The list of host statuses
    #[serde(rename = "status")]
    pub statuses: Vec<ObjectStatus>,
    /// The list of host IP addresses
    #[serde(rename = "addr")]
    pub addresses: Vec<HostAddr>,
    /// The epp user to whom the host belongs
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// THe epp user that created the host
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The epp user that last updated the host
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    /// The host last update date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    /// The host transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
}

/// Type that represents the &lt;resData&gt; tag for host info response
#[derive(Deserialize, Debug)]
pub struct HostInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: HostInfoResponseData,
}

#[cfg(test)]
mod tests {
    use super::HostInfo;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/info.xml").unwrap();

        let object = HostInfo::new("ns1.eppdev-1.com");

        let serialized =
            <HostInfo as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/info.xml").unwrap();
        let object =
            <HostInfo as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.info_data.name, "host2.eppdev-1.com".into());
        assert_eq!(result.info_data.roid, "UNDEF-ROID".into());
        assert_eq!(result.info_data.statuses[0].status, "ok".to_string());
        assert_eq!(
            *(result.info_data.addresses[0].ip_version.as_ref().unwrap()),
            "v4".to_string()
        );
        assert_eq!(
            result.info_data.addresses[0].address,
            "29.245.122.14".to_string()
        );
        assert_eq!(
            *(result.info_data.addresses[1].ip_version.as_ref().unwrap()),
            "v6".to_string()
        );
        assert_eq!(
            result.info_data.addresses[1].address,
            "2404:6800:4001:0801:0000:0000:0000:200e".to_string()
        );
        assert_eq!(result.info_data.client_id, "eppdev".into());
        assert_eq!(result.info_data.creator_id, "creator".into());
        assert_eq!(result.info_data.created_at, "2021-07-26T05:28:55.0Z".into());
        assert_eq!(
            *(result.info_data.updater_id.as_ref().unwrap()),
            "creator".into()
        );
        assert_eq!(
            *(result.info_data.updated_at.as_ref().unwrap()),
            "2021-07-26T05:28:55.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
