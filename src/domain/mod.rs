use std::borrow::Cow;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use instant_xml::OptionAccumulator;
use instant_xml::{Accumulate, Deserializer, FromXml, Serializer, ToXml};

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
#[derive(Clone, Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(rename = "hostAttr", ns(XMLNS))]
pub struct HostAttr<'a> {
    /// The &lt;hostName&gt; tag
    #[xml(rename = "hostName")]
    pub name: Cow<'a, str>,
    /// The &lt;hostAddr&gt; tags
    #[xml(
        rename = "hostAddr",
        serialize_with = "serialize_host_addrs_option",
        deserialize_with = "deserialize_host_addrs_option"
    )]
    pub addresses: Option<Vec<IpAddr>>,
}

fn deserialize_host_addrs_option<'xml>(
    into: &mut OptionAccumulator<Vec<IpAddr>, Vec<IpAddr>>,
    field: &'static str,
    deserializer: &mut Deserializer<'_, 'xml>,
) -> Result<(), instant_xml::Error> {
    let mut value = <Option<Vec<HostAddr<'static>>> as FromXml<'xml>>::Accumulator::default();
    <Option<Vec<HostAddr<'static>>>>::deserialize(&mut value, field, deserializer)?;
    let new = match value.try_done(field)? {
        Some(new) => new,
        None => return Ok(()),
    };

    let into = into.get_mut();
    for addr in new {
        match IpAddr::from_str(&addr.address) {
            Ok(ip) => into.push(ip),
            Err(_) => {
                return Err(instant_xml::Error::UnexpectedValue(format!(
                    "invalid IP address '{}'",
                    &addr.address
                )))
            }
        }
    }

    Ok(())
}

/// The &lt;hostAddr&gt; types domain or host transactions
#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "hostAddr", ns(super::domain::XMLNS))]
pub(crate) struct HostAddr<'a> {
    #[xml(attribute, rename = "ip")]
    pub ip_version: Option<Cow<'a, str>>,
    #[xml(direct)]
    pub address: Cow<'a, str>,
}

impl From<&IpAddr> for HostAddr<'static> {
    fn from(addr: &IpAddr) -> Self {
        Self {
            ip_version: Some(match addr {
                IpAddr::V4(_) => "v4".into(),
                IpAddr::V6(_) => "v6".into(),
            }),
            address: addr.to_string().into(),
        }
    }
}

pub(crate) fn serialize_host_addrs_option<T: AsRef<[IpAddr]>, W: fmt::Write + ?Sized>(
    addrs: &Option<T>,
    serializer: &mut Serializer<'_, W>,
) -> Result<(), instant_xml::Error> {
    let addrs = match addrs {
        Some(addrs) => addrs.as_ref(),
        None => return Ok(()),
    };

    for addr in addrs {
        HostAddr::from(addr).serialize(None, serializer)?;
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(rename = "hostObj", ns(XMLNS))]
pub struct HostObj<'a> {
    #[xml(direct)]
    pub name: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(forward)]
pub enum HostInfo<'a> {
    Attr(HostAttr<'a>),
    Obj(HostObj<'a>),
}

#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "ns", ns(XMLNS))]
pub struct NameServers<'a> {
    pub ns: Vec<HostInfo<'a>>,
}

/// The &lt;contact&gt; type on domain creation and update requests
#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "contact", ns(XMLNS))]
pub struct DomainContact<'a> {
    /// The contact type attr (usually admin, billing, or tech in most registries)
    #[xml(attribute, rename = "type")]
    pub contact_type: Cow<'a, str>,
    /// The contact id
    #[xml(direct)]
    pub id: Cow<'a, str>,
}

/// The &lt;period&gt; type for registration, renewal or transfer on domain transactions
#[derive(Clone, Copy, Debug, ToXml)]
#[xml(rename = "period", ns(XMLNS))]
pub struct Period {
    /// The interval (usually 'y' indicating years)
    #[xml(attribute)]
    unit: char,
    /// The length of the registration, renewal or transfer period (usually in years)
    #[xml(direct)]
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

pub const ONE_YEAR: Period = Period {
    unit: 'y',
    length: 1,
};

pub const TWO_YEARS: Period = Period {
    unit: 'y',
    length: 2,
};

pub const THREE_YEARS: Period = Period {
    unit: 'y',
    length: 3,
};

pub const ONE_MONTH: Period = Period {
    unit: 'm',
    length: 1,
};

pub const SIX_MONTHS: Period = Period {
    unit: 'm',
    length: 6,
};

/// The &lt;authInfo&gt; tag for domain and contact transactions
#[derive(Clone, Debug, FromXml, ToXml)]
#[xml(rename = "authInfo", ns(XMLNS))]
pub struct DomainAuthInfo<'a> {
    /// The &lt;pw&gt; tag under &lt;authInfo&gt;
    #[xml(rename = "pw")]
    pub password: Cow<'a, str>,
}

impl<'a> DomainAuthInfo<'a> {
    /// Creates a DomainAuthInfo instance with the given password
    pub fn new(password: &'a str) -> Self {
        Self {
            password: password.into(),
        }
    }
}

/// The &lt;status&gt; type on contact transactions
#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "status", ns(XMLNS))]
pub struct Status<'a> {
    /// The status name, represented by the 's' attr on &lt;status&gt; tags
    #[xml(attribute, rename = "s")]
    pub status: Cow<'a, str>,
}
