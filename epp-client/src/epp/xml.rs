//! Types to use in serialization to and deserialization from EPP XML

pub mod quick_xml;

use std::{error::Error, fmt::Debug};

use crate::error;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;
pub const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";

pub const EPP_DOMAIN_XMLNS: &str = "urn:ietf:params:xml:ns:domain-1.0";
pub const EPP_CONTACT_XMLNS: &str = "urn:ietf:params:xml:ns:contact-1.0";
pub const EPP_HOST_XMLNS: &str = "urn:ietf:params:xml:ns:host-1.0";

pub const EPP_DOMAIN_RGP_EXT_XMLNS: &str = "urn:ietf:params:xml:ns:rgp-1.0";

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

/// Trait to be implemented by serializers. Currently the only included serializer is `quick-xml`
pub trait EppXml {
    type Output: Debug;

    fn serialize(&self) -> Result<String, Box<dyn Error>>;
    fn deserialize(epp_xml: &str) -> Result<Self::Output, error::Error>;
}
