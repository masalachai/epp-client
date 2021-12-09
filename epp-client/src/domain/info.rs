//! Types for EPP domain info request

use super::XMLNS;
use crate::common::{
    DomainAuthInfo, DomainContact, DomainStatus, HostAttr, NoExtension, StringValue,
};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainInfo {}

impl Command for DomainInfo {
    type Response = DomainInfoResponse;
    const COMMAND: &'static str = "info";
}

impl DomainInfo {
    pub fn new(name: &str, auth_password: Option<&str>) -> Self {
        Self {
            info: DomainInfoRequestData {
                xmlns: XMLNS.to_string(),
                domain: Domain {
                    hosts: "all".to_string(),
                    name: name.to_string(),
                },
                auth_info: auth_password.map(|password| DomainAuthInfo {
                    password: password.into(),
                }),
            },
        }
    }
}

// Request

/// Type for data under the &lt;name&gt; element tag for the domain &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct Domain {
    /// The hosts attribute. Default value is "all"
    hosts: String,
    /// The name of the domain
    #[serde(rename = "$value")]
    name: String,
}

/// Type for &lt;name&gt; element under the domain &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainInfoRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The data for the domain to be queried
    #[serde(rename = "domain:name", alias = "name")]
    domain: Domain,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    auth_info: Option<DomainAuthInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;info&gt; command for domains
pub struct DomainInfo {
    /// The data under the &lt;info&gt; tag for domain info
    #[serde(rename = "domain:info", alias = "info")]
    info: DomainInfoRequestData,
}

// Response

/// The two types of ns lists, hostObj and hostAttr, that may be returned in the
/// domain info response
#[derive(Deserialize, Debug)]
pub struct DomainNsList {
    /// List of &lt;hostObj&gt; ns elements
    #[serde(rename = "hostObj")]
    pub host_obj: Option<Vec<StringValue>>,
    /// List of &lt;hostAttr&gt; ns elements
    pub host_attr: Option<Vec<HostAttr>>,
}

/// Type that represents the &lt;infData&gt; tag for domain info response
#[derive(Deserialize, Debug)]
pub struct DomainInfoResponseData {
    /// The domain name
    pub name: StringValue,
    /// The domain ROID
    pub roid: StringValue,
    /// The list of domain statuses
    #[serde(rename = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
    /// The domain registrant
    pub registrant: Option<StringValue>,
    /// The list of domain contacts
    #[serde(rename = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of domain nameservers
    #[serde(rename = "ns")]
    pub ns: Option<DomainNsList>,
    /// The list of domain hosts
    #[serde(rename = "host")]
    pub hosts: Option<Vec<StringValue>>,
    /// The epp user who owns the domain
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// The epp user who created the domain
    #[serde(rename = "crID")]
    pub creator_id: Option<StringValue>,
    /// The domain creation date
    #[serde(rename = "crDate")]
    pub created_at: Option<StringValue>,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: Option<StringValue>,
    /// The epp user who last updated the domain
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    /// The domain last updated date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    /// The domain transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    /// The domain auth info
    #[serde(rename = "authInfo")]
    pub auth_info: Option<DomainAuthInfo>,
}

/// Type that represents the &lt;resData&gt; tag for domain info response
#[derive(Deserialize, Debug)]
pub struct DomainInfoResponse {
    /// Data under the &lt;resData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: DomainInfoResponseData,
}
