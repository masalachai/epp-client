//! Types for EPP host create request

use super::XMLNS;
use crate::common::{HostAddr, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for HostCreate {}

impl Command for HostCreate {
    type Response = HostCreateResponse;
    const COMMAND: &'static str = "create";
}

impl HostCreate {
    pub fn new(host: &str, addresses: Vec<HostAddr>) -> Self {
        Self {
            host: HostCreateRequestData {
                xmlns: XMLNS.to_string(),
                name: host.into(),
                addresses: Some(addresses),
            },
        }
    }
}

// Request

/// Type for data under the host &lt;create&gt; tag
#[derive(Serialize, Debug)]
pub struct HostCreateRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host to be created
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
    /// The list of IP addresses for the host
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;create&gt; command for hosts
pub struct HostCreate {
    /// The instance holding the data for the host to be created
    #[serde(rename = "host:create", alias = "create")]
    host: HostCreateRequestData,
}

// Response

/// Type that represents the &lt;creData&gt; tag for host create response
#[derive(Deserialize, Debug)]
pub struct HostCreateData {
    /// The host name
    pub name: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct HostCreateResponse {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: HostCreateData,
}
