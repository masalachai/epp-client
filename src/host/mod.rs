use std::borrow::Cow;
use std::fmt;
use std::net::IpAddr;

use instant_xml::{FromXml, Serializer, ToXml};

pub mod check;
pub use check::HostCheck;

pub mod create;
pub use create::HostCreate;

pub mod delete;
pub use delete::HostDelete;

pub mod info;
pub use info::HostInfo;

pub mod update;
pub use update::HostUpdate;

pub const XMLNS: &str = "urn:ietf:params:xml:ns:host-1.0";

/// The &lt;status&gt; type on contact transactions
#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "status", ns(XMLNS))]
pub struct Status<'a> {
    /// The status name, represented by the 's' attr on &lt;status&gt; tags
    #[xml(attribute, rename = "s")]
    pub status: Cow<'a, str>,
}

/// The &lt;hostAddr&gt; types domain or host transactions
#[derive(Debug, FromXml, ToXml)]
#[xml(rename = "addr", ns(XMLNS))]
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
