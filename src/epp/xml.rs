use std::error::Error;

// use crate::epp::object::EppObject;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;
pub const EPP_XMLNS: &str = "urn:ietf:params:xml:ns:epp-1.0";
pub const EPP_XMLNS_XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";
pub const EPP_XSI_SCHEMA_LOCATION: &str = "urn:ietf:params:xml:ns:epp-1.0 epp-1.0.xsd";

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

pub trait EppXml {
    type Object;

    fn serialize(&self) -> Result<String, Box<dyn Error>>;
    fn deserialize(epp_xml: &str) -> Result<Self::Object, Box<dyn Error>>;
}
