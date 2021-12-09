//! Types for EPP host check request

use std::fmt::Debug;

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for HostCheck {}

impl Command for HostCheck {
    type Response = HostCheckResponse;
    const COMMAND: &'static str = "check";
}

impl HostCheck {
    pub fn new(hosts: &[&str]) -> Self {
        let hosts = hosts.iter().map(|&d| d.into()).collect();

        Self {
            list: HostList {
                xmlns: XMLNS.to_string(),
                hosts,
            },
        }
    }
}

// Request

/// Type for data under the host &lt;check&gt; tag
#[derive(Serialize, Debug)]
pub struct HostList {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// List of hosts to be checked for availability
    #[serde(rename = "host:name", alias = "name")]
    pub hosts: Vec<StringValue>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;check&gt; command for hosts
pub struct HostCheck {
    /// The instance holding the list of hosts to be checked
    #[serde(rename = "host:check", alias = "check")]
    list: HostList,
}

// Response

/// Type that represents the &lt;name&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct HostAvailable {
    /// The host name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The host (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct HostCheckDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub host: HostAvailable,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct HostCheckData {
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub host_list: Vec<HostCheckDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct HostCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: HostCheckData,
}
