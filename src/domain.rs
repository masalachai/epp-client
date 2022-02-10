use std::borrow::Cow;
use std::net::IpAddr;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::common::{serialize_host_addrs_option, HostAddr, StringValue};
use crate::Error;

pub mod check;
pub use check::DomainCheck;

pub mod create;
pub use create::DomainCreate;

pub mod delete;
pub use delete::DomainDelete;

pub mod info;
pub use info::DomainInfo;

pub mod renew;
pub use renew::DomainRenew;

pub mod transfer;
pub use transfer::DomainTransfer;

pub mod update;
pub use update::DomainUpdate;

pub const XMLNS: &str = "urn:ietf:params:xml:ns:domain-1.0";

/// The &lt;hostAttr&gt; type for domain transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttr<'a> {
    /// The &lt;hostName&gt; tag
    #[serde(rename = "domain:hostName", alias = "hostName")]
    pub name: StringValue<'a>,
    /// The &lt;hostAddr&gt; tags
    #[serde(
        rename = "domain:hostAddr",
        alias = "hostAddr",
        serialize_with = "serialize_host_addrs_option",
        deserialize_with = "deserialize_host_addrs_option"
    )]
    pub addresses: Option<Vec<IpAddr>>,
}

fn deserialize_host_addrs_option<'de, D>(de: D) -> Result<Option<Vec<IpAddr>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let addrs = Option::<Vec<HostAddr<'static>>>::deserialize(de)?;
    let addrs = match addrs {
        Some(addrs) => addrs,
        None => return Ok(None),
    };

    let result = addrs
        .into_iter()
        .map(|addr| IpAddr::from_str(&addr.address))
        .collect::<Result<_, _>>();

    match result {
        Ok(addrs) => Ok(Some(addrs)),
        Err(e) => Err(serde::de::Error::custom(format!("{}", e))),
    }
}

/// The list of &lt;hostAttr&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Debug)]
pub struct HostAttrList<'a> {
    /// The list of &lt;hostAttr&gt; tags
    #[serde(rename = "domain:hostAttr", alias = "hostAttr")]
    pub hosts: &'a [HostAttr<'a>],
}

/// The list of &lt;hostObj&gt; types for domain transactions. Typically under an &lt;ns&gt; tag
#[derive(Serialize, Debug)]
pub struct HostObjList<'a> {
    /// The list of &lt;hostObj&gt; tags
    #[serde(rename = "domain:hostObj", alias = "hostObj")]
    pub hosts: &'a [StringValue<'a>],
}

/// Enum that can accept one type which corresponds to either the &lt;hostObj&gt; or &lt;hostAttr&gt;
/// list of tags
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum HostList<'a> {
    HostObjList(HostObjList<'a>),
    HostAttrList(HostAttrList<'a>),
}

/// The &lt;contact&gt; type on domain creation and update requests
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainContact<'a> {
    /// The contact id
    #[serde(rename = "$value")]
    pub id: Cow<'a, str>,
    /// The contact type attr (usually admin, billing, or tech in most registries)
    #[serde(rename = "type")]
    pub contact_type: Cow<'a, str>,
}

/// The &lt;period&gt; type for registration, renewal or transfer on domain transactions
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Period {
    /// The interval (usually 'y' indicating years)
    unit: char,
    /// The length of the registration, renewal or transfer period (usually in years)
    #[serde(rename = "$value")]
    length: u8,
}

impl Period {
    pub fn years(length: u8) -> Result<Self, Error> {
        Self::new(length, 'y')
    }

    pub fn months(length: u8) -> Result<Self, Error> {
        Self::new(length, 'm')
    }

    fn new(length: u8, unit: char) -> Result<Self, Error> {
        match length {
            1..=99 => Ok(Period { length, unit }),
            0 | 100.. => Err(Error::Other(
                "Period length must be greater than 0 and less than 100".into(),
            )),
        }
    }
}

/// The &lt;authInfo&gt; tag for domain and contact transactions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainAuthInfo<'a> {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[serde(rename = "domain:pw", alias = "pw")]
    pub password: StringValue<'a>,
}

impl<'a> DomainAuthInfo<'a> {
    /// Creates a DomainAuthInfo instance with the given password
    pub fn new(password: &'a str) -> Self {
        Self {
            password: password.into(),
        }
    }
}
