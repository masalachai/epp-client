//! Types for EPP host delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl Transaction<NoExtension> for HostDelete {}

impl Command for HostDelete {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

impl HostDelete {
    pub fn new(name: &str) -> Self {
        Self {
            host: HostDeleteRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
            },
        }
    }
}

/// Type for data under the host &lt;delete&gt; tag
#[derive(Serialize, Debug)]
pub struct HostDeleteRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The host to be deleted
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;delete&gt; command for hosts
pub struct HostDelete {
    /// The instance holding the data for the host to be deleted
    #[serde(rename = "host:delete", alias = "delete")]
    host: HostDeleteRequestData,
}
