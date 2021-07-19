use quick_xml::se;
use std::error::Error;

use crate::epp::request::EppObject;
use crate::epp::xml::{EppXml, EPP_XML_HEADER};

impl EppXml for EppObject {
    fn serialize(&self) -> Result<String, Box<dyn Error>> {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }
}
