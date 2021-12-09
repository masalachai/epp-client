//! Types for EPP domain create request

use super::XMLNS;
use crate::common::{DomainAuthInfo, DomainContact, HostList, NoExtension, Period, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainCreate {}

impl Command for DomainCreate {
    type Response = DomainCreateResponse;
    const COMMAND: &'static str = "create";
}

// Request

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainCreateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    pub xmlns: String,
    /// The domain name
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// The period of registration
    #[serde(rename = "domain:period", alias = "period")]
    pub period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    pub ns: Option<HostList>,
    /// The domain registrant
    #[serde(rename = "domain:registrant", alias = "registrant")]
    pub registrant: Option<StringValue>,
    /// The list of contacts for the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    pub auth_info: DomainAuthInfo,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;create&gt; command for domains
pub struct DomainCreate {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "domain:create", alias = "create")]
    pub domain: DomainCreateRequestData,
}

impl DomainCreate {
    pub fn new(
        name: &str,
        period: u16,
        ns: Option<HostList>,
        registrant_id: Option<&str>,
        auth_password: &str,
        contacts: Option<Vec<DomainContact>>,
    ) -> Self {
        Self {
            domain: DomainCreateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns,
                registrant: registrant_id.map(|id| id.into()),
                auth_info: DomainAuthInfo::new(auth_password),
                contacts,
            },
        }
    }
}

// Response

/// Type that represents the &lt;chkData&gt; tag for domain create response
#[derive(Deserialize, Debug)]
pub struct DomainCreateResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    pub xmlns: String,
    /// The domain name
    pub name: StringValue,
    /// The creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain create response
#[derive(Deserialize, Debug)]
pub struct DomainCreateResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: DomainCreateResponseData,
}
