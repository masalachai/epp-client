//! Types to use in serialization to and deserialization from EPP XML

use quick_xml::de::from_str;
use quick_xml::se;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

use crate::common::{ElementName, EppObject};
use crate::error;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

impl<T: Serialize + DeserializeOwned + ElementName + Debug> EppXml for EppObject<T> {
    type Output = EppObject<T>;

    /// Serializes the EppObject instance to an EPP XML document
    fn serialize(&self) -> Result<String, Box<dyn Error>> {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }

    /// Deserializes an EPP XML document to an EppObject instance
    fn deserialize(epp_xml: &str) -> Result<Self::Output, error::Error> {
        let object: Self::Output = match from_str(epp_xml) {
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

/// Trait to be implemented by serializers. Currently the only included serializer is `quick-xml`
pub trait EppXml {
    type Output: Debug;

    fn serialize(&self) -> Result<String, Box<dyn Error>>;
    fn deserialize(epp_xml: &str) -> Result<Self::Output, error::Error>;
}
