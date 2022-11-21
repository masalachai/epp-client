//! Common data types included in EPP Requests and Responses

use std::borrow::Cow;

use instant_xml::{FromXml, ToXml};

use crate::request::Extension;

pub(crate) const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";

#[derive(Debug, Eq, PartialEq, ToXml)]
pub struct NoExtension;

impl<'xml> FromXml<'xml> for NoExtension {
    fn matches(_: instant_xml::Id<'_>, _: Option<instant_xml::Id<'_>>) -> bool {
        false
    }

    fn deserialize<'cx>(
        _: &mut Self::Accumulator,
        _: &'static str,
        _: &mut instant_xml::Deserializer<'cx, 'xml>,
    ) -> Result<(), instant_xml::Error> {
        unreachable!()
    }

    type Accumulator = Option<Self>;
    const KIND: instant_xml::Kind = instant_xml::Kind::Element;
}

impl Extension for NoExtension {
    type Response = NoExtension;
}

/// The <option> type in EPP XML login requests
#[derive(Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(rename = "options", ns(EPP_XMLNS))]
pub struct Options<'a> {
    /// The EPP version being used
    pub version: Cow<'a, str>,
    /// The language that will be used during EPP transactions
    pub lang: Cow<'a, str>,
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
#[derive(Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(rename = "svcExtension", ns(EPP_XMLNS))]
pub struct ServiceExtension<'a> {
    /// The service extension URIs being represented by <extURI> in EPP XML
    #[xml(rename = "extURI")]
    pub ext_uris: Option<Vec<Cow<'a, str>>>,
}

/// The <svcs> type in EPP XML
#[derive(Debug, Eq, FromXml, PartialEq, ToXml)]
#[xml(rename = "svcs", ns(EPP_XMLNS))]
pub struct Services<'a> {
    /// The service URIs being used by this EPP session represented by <objURI> in EPP XML
    #[xml(rename = "objURI")]
    pub obj_uris: Vec<Cow<'a, str>>,
    // The <svcExtension> being used in this EPP session
    #[xml(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension<'a>>,
}

/// This type contains a single DER-encoded X.509 certificate.
///
/// The rustls-pemfile crate can be used to parse a PEM file.
pub struct Certificate(pub Vec<u8>);

/// This type contains a DER-encoded ASN.1 private key in PKCS#8 or PKCS#1 format.
///
/// The rustls-pemfile crate can be used to parse a PEM file in these formats.
pub struct PrivateKey(pub Vec<u8>);
