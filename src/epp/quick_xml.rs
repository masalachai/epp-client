use quick_xml::se;
use serde::Serialize;
use std::error::Error;

use crate::epp::object::EppObject;
use crate::epp::xml::{EppXml, EPP_XML_HEADER};

impl<T: Serialize> EppXml for EppObject<T> {
    fn serialize(&self) -> Result<String, Box<dyn Error>> {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }
}
