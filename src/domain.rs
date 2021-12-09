use serde::{Deserialize, Serialize};

use crate::common::{HostAddr, StringValue};

pub mod check;
pub mod create;
pub mod delete;
pub mod info;
pub mod renew;
pub mod transfer;
pub mod update;

pub const XMLNS: &str = "urn:ietf:params:xml:ns:domain-1.0";

/// The &lt;hostAttr&gt; type for domain transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttr {
    /// The &lt;hostName&gt; tag
    #[serde(rename = "domain:hostName", alias = "hostName")]
    pub name: StringValue,
    /// The &lt;hostAddr&gt; tags
    #[serde(rename = "domain:hostAddr", alias = "hostAddr")]
    pub addresses: Option<Vec<HostAddr>>,
}

/// The list of &lt;hostAttr&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttrList {
    /// The list of &lt;hostAttr&gt; tags
    #[serde(rename = "domain:hostAttr", alias = "hostAttr")]
    pub hosts: Vec<HostAttr>,
}

/// The list of &lt;hostObj&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostObjList {
    /// The list of &lt;hostObj&gt; tags
    #[serde(rename = "domain:hostObj", alias = "hostObj")]
    pub hosts: Vec<StringValue>,
}

/// Enum that can accept one type which corresponds to either the &lt;hostObj&gt; or &lt;hostAttr&gt;
/// list of tags
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HostList {
    HostObjList(HostObjList),
    HostAttrList(HostAttrList),
}

/// The &lt;contact&gt; type on domain creation and update requests
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainContact {
    /// The contact id
    #[serde(rename = "$value")]
    pub id: String,
    /// The contact type attr (usually admin, billing, or tech in most registries)
    #[serde(rename = "type")]
    pub contact_type: String,
}

/// The &lt;period&gt; type for registration, renewal or transfer on domain transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    /// The interval (usually 'y' indicating years)
    unit: String,
    /// The length of the registration, renewal or transfer period (usually in years)
    #[serde(rename = "$value")]
    length: u16,
}

impl Period {
    /// Creates a new period in years
    pub fn new(length: u16) -> Period {
        Period {
            unit: "y".to_string(),
            length,
        }
    }

    /// Sets the period unit ('y' for years, most commonly)
    pub fn set_unit(&mut self, unit: &str) {
        self.unit = unit.to_string();
    }
}

/// The &lt;authInfo&gt; tag for domain and contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainAuthInfo {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "domain:pw", alias = "pw")]
    pub password: StringValue,
}

impl DomainAuthInfo {
    /// Creates a DomainAuthInfo instance with the given password
    pub fn new(password: &str) -> DomainAuthInfo {
        DomainAuthInfo {
            password: password.into(),
        }
    }
}
