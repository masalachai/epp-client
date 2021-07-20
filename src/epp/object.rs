use serde::{Deserialize, Serialize};

use crate::epp::xml::{EPP_XMLNS, EPP_XMLNS_XSI, EPP_XSI_SCHEMA_LOCATION};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StringValue(String);

impl Default for StringValue {
    fn default() -> Self {
        Self(String::from(""))
    }
}

pub trait StringValueTrait {
    fn to_string_value(&self) -> StringValue;
}

impl StringValueTrait for &str {
    fn to_string_value(&self) -> StringValue {
        StringValue(self.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct EppObject<T> {
    pub xmlns: String,
    #[serde(rename = "xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "xsi:schemaLocation")]
	pub xsi_schema_location: String,
	#[serde(alias = "greeting")]
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "options")]
pub struct Options {
    pub version: StringValue,
    pub lang: StringValue,
}

impl Options {
    pub fn build(version: &str, lang: &str) -> Options {
        Options {
            version: version.to_string_value(),
            lang: lang.to_string_value(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "svcExtension")]
pub struct ServiceExtension {
    #[serde(rename = "extURI")]
    pub ext_uris: Option<Vec<StringValue>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Services {
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue>,
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension>,
}

impl<T> EppObject<T> {
    pub fn new(data: T) -> EppObject<T> {
        EppObject {
            data: data,
            xmlns: EPP_XMLNS.to_string(),
            xmlns_xsi: EPP_XMLNS_XSI.to_string(),
            xsi_schema_location: EPP_XSI_SCHEMA_LOCATION.to_string(),
        }
    }
}
