//! Common data types included in EPP Requests and Responses

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::request::Extension;

pub(crate) const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";

/// Wraps String for easier serialization to and from values that are inner text
/// for tags rather than attributes
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StringValue<'a>(Cow<'a, str>);

impl Display for StringValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for StringValue<'a> {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl From<String> for StringValue<'static> {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// An empty placeholder tag. To be refactored to something more compliant later.
pub struct NoExtension;

impl Extension for NoExtension {
    type Response = NoExtension;
}

/// Type that represents the &lt;name&gt; tag for host check response
#[derive(Deserialize, Debug)]
pub struct Available {
    /// The resource name
    #[serde(rename = "$value")]
    pub name: StringValue<'static>,
    /// The resource (un)availability
    #[serde(rename = "avail")]
    pub available: bool,
}

/// The <option> type in EPP XML login requests
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "options")]
pub struct Options<'a> {
    /// The EPP version being used
    pub version: StringValue<'a>,
    /// The language that will be used during EPP transactions
    pub lang: StringValue<'a>,
}

impl<'a> Options<'a> {
    /// Creates an Options object with version and lang data
    pub fn build(version: &'a str, lang: &'a str) -> Self {
        Self {
            version: version.into(),
            lang: lang.into(),
        }
    }
}

/// The <svcExtension> type in EPP XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "svcExtension")]
pub struct ServiceExtension<'a> {
    /// The service extension URIs being represented by <extURI> in EPP XML
    #[serde(rename = "extURI")]
    pub ext_uris: Option<Vec<StringValue<'a>>>,
}

/// The <svcs> type in EPP XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Services<'a> {
    /// The service URIs being used by this EPP session represented by <objURI> in EPP XML
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue<'a>>,
    /// The <svcExtention> being used in this EPP session
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension<'a>>,
}

/// The &lt;hostAddr&gt; types domain or host transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddr<'a> {
    #[serde(rename = "ip")]
    pub ip_version: Option<Cow<'a, str>>,
    #[serde(rename = "$value")]
    pub address: Cow<'a, str>,
}

impl<'a> HostAddr<'a> {
    /// Creates a 'v4' type HostAddr (mostly useful when you don't want to include an 'ip' attr in the XML)
    pub fn new(ip_version: &'a str, address: &'a str) -> Self {
        Self {
            ip_version: Some(ip_version.into()),
            address: address.into(),
        }
    }

    /// Creates a 'v4' type HostAddr
    pub fn new_v4(address: &'a str) -> HostAddr {
        HostAddr {
            ip_version: Some("v4".into()),
            address: address.into(),
        }
    }

    /// Creates a 'v6' type HostAddr
    pub fn new_v6(address: &'a str) -> HostAddr {
        HostAddr {
            ip_version: Some("v6".into()),
            address: address.into(),
        }
    }
}

/// The &lt;status&gt; type on contact transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectStatus<'a> {
    /// The status name, represented by the 's' attr on &lt;status&gt; tags
    #[serde(rename = "s")]
    pub status: Cow<'a, str>,
}

/// This type contains a single DER-encoded X.509 certificate.
///
/// The rustls-pemfile crate can be used to parse a PEM file.
pub struct Certificate(pub Vec<u8>);

/// This type contains a DER-encoded ASN.1 private key in PKCS#8 or PKCS#1 format.
///
/// The rustls-pemfile crate can be used to parse a PEM file in these formats.
pub struct PrivateKey(pub Vec<u8>);
