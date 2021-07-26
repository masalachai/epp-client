//! XML serialization using the `quick-xml` library

use quick_xml::de::from_str;
use quick_xml::se;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

use crate::epp::object::{ElementName, EppObject};
use crate::epp::xml::{EppXml, EPP_XML_HEADER};
use crate::error;

impl<T: Serialize + DeserializeOwned + ElementName + Debug> EppXml for EppObject<T> {
    type Output = EppObject<T>;

    fn serialize(&self) -> Result<String, Box<dyn Error>> {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }

    fn deserialize(epp_xml: &str) -> Result<Self::Output, error::Error> {
        let object: Self::Output = match from_str(epp_xml) {
            Ok(v) => v,
            Err(e) => {
                return Err(error::Error::EppDeserializationError(
                    format!("epp-client Deserialization Error: {}", e).to_string(),
                ))
            }
        };
        // object.xml = Some(epp_xml.to_string());
        Ok(object)
    }
}
