//! Types to use in serialization to and deserialization from EPP XML

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

/// Trait to be implemented by serializers. Currently the only included serializer is `quick-xml`
pub trait EppXml: Sized {
    /// Serializes the EppObject instance to an EPP XML document
    fn serialize(&self) -> Result<String, Error>
    where
        Self: Serialize,
    {
        Ok(format!(
            "{}\r\n{}",
            EPP_XML_HEADER,
            quick_xml::se::to_string(self).map_err(|e| Error::Xml(e.into()))?
        ))
    }

    /// Deserializes an EPP XML document to an EppObject instance
    fn deserialize(epp_xml: &str) -> Result<Self, Error>
    where
        Self: DeserializeOwned + Sized,
    {
        quick_xml::de::from_str::<Self>(epp_xml).map_err(|e| Error::Xml(e.into()))
    }
}
