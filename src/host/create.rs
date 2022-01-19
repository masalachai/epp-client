//! Types for EPP host create request

use super::XMLNS;
use crate::common::{HostAddr, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl<'a> Transaction<NoExtension> for HostCreate<'a> {}

impl<'a> Command for HostCreate<'a> {
    type Response = HostCreateResponse;
    const COMMAND: &'static str = "create";
}

impl<'a> HostCreate<'a> {
    pub fn new(host: &'a str, addresses: Option<&'a [HostAddr]>) -> Self {
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
    #[serde(rename = "host:addr")]
    pub addresses: Option<&'a [HostAddr<'a>]>,
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
    pub created_at: StringValue<'static>,
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
    use super::HostCreate;
    use crate::common::{HostAddr, NoExtension};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/host/create.xml").unwrap();

        let addresses = &[
            HostAddr::new("v4", "29.245.122.14"),
            HostAddr::new("v6", "2404:6800:4001:801::200e"),
        ];

        let object = HostCreate::new("host1.eppdev-1.com", Some(addresses));

        let serialized =
            <HostCreate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/host/create.xml").unwrap();
        let object =
            <HostCreate as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.create_data.name, "host2.eppdev-1.com".into());
        assert_eq!(
            result.create_data.created_at,
            "2021-07-26T05:28:55.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
