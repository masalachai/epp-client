use std::error::Error;

pub const EPP_XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#;

pub trait EppXml {
    fn serialize(&self) -> Result<String, Box<dyn Error>>;
    // fn deserialize(&self) -> Result<Self, Box<dyn Error>>;
}
