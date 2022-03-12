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
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let object = HostDelete::new("ns1.eppdev-1.com");
        assert_serialized("request/host/delete.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<HostDelete>("response/host/delete.xml");
        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
