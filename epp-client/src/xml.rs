//! Types to use in serialization to and deserialization from EPP XML

use quick_xml::de::from_str;
use quick_xml::se;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

use crate::error;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

/// Trait to be implemented by serializers. Currently the only included serializer is `quick-xml`
pub trait EppXml: Sized {
    /// Serializes the EppObject instance to an EPP XML document
    fn serialize(&self) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }

    /// Deserializes an EPP XML document to an EppObject instance
    fn deserialize(epp_xml: &str) -> Result<Self, error::Error>
    where
        Self: DeserializeOwned + Sized,
    {
        let object: Self = match from_str(epp_xml) {
            Ok(v) => v,
            Err(e) => {
                return Err(error::Error::EppDeserializationError(format!(
                    "epp-client Deserialization Error: {}",
                    e
                )))
            }
        };
        // object.xml = Some(epp_xml.to_string());
        Ok(object)
    }
}
