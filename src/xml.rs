//! Types to use in serialization to and deserialization from EPP XML

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

pub(crate) fn serialize(doc: &impl Serialize) -> Result<String, Error> {
    Ok(format!(
        "{}\r\n{}",
        EPP_XML_HEADER,
        quick_xml::se::to_string(doc).map_err(|e| Error::Xml(e.into()))?
    ))
}

pub(crate) fn deserialize<T: DeserializeOwned>(xml: &str) -> Result<T, Error> {
    quick_xml::de::from_str(xml).map_err(|e| Error::Xml(e.into()))
}
