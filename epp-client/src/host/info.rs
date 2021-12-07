//! Types for EPP host info request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, HostAddr, HostStatus, NoExtension, StringValue};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostInfo<E> {
    request: HostInfoRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for HostInfo<E> {
    type Input = HostInfoRequest;
    type Output = HostInfoResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> HostInfo<E> {
    pub fn new(name: &str) -> HostInfo<NoExtension> {
        HostInfo {
            request: HostInfoRequest {
                info: HostInfoRequestData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostInfo<F> {
        HostInfo {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for data under the host &lt;info&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host to be queried
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML &lt;info&gt; command for hosts
pub struct HostInfoRequest {
    /// The instance holding the data for the host query
    #[serde(rename = "host:info", alias = "info")]
    info: HostInfoRequestData,
}

// Response

/// Type that represents the &lt;infData&gt; tag for host info response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoResponseData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// The host name
    pub name: StringValue,
    /// The host ROID
    pub roid: StringValue,
    /// The list of host statuses
    #[serde(rename = "status")]
    pub statuses: Vec<HostStatus>,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct HostInfoResponse {
    /// Data under the &lt;infData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: HostInfoResponseData,
}
