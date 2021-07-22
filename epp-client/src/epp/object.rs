pub mod data;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::fmt::Display;

use crate::epp::xml::{EPP_XMLNS, EPP_XMLNS_XSI, EPP_XSI_SCHEMA_LOCATION};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StringValue(String);

impl Default for StringValue {
    fn default() -> Self {
        Self(String::from(""))
    }
}

impl Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

pub trait ElementName {
    fn element_name(&self) -> &'static str;
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct EppObject<T: ElementName> {
    pub xmlns: String,
    #[serde(rename = "xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "xsi:schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(alias = "greeting", alias = "response")]
    pub data: T,
    // #[serde(skip)]
    // pub xml: Option<String>,
}

impl<T: ElementName + Serialize> Serialize for EppObject<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data_name = self.data.element_name();
        let mut state = serializer.serialize_struct("epp", 4)?;
        state.serialize_field("xmlns", &self.xmlns)?;
        state.serialize_field("xmlns:xsi", &self.xmlns_xsi)?;
        state.serialize_field("xsi:schemaLocation", &self.xsi_schema_location)?;
        state.serialize_field(data_name, &self.data)?;
        state.end()
    }
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

impl<T: ElementName> EppObject<T> {
    pub fn build(data: T) -> EppObject<T> {
        EppObject {
            // xml: None,
            data: data,
            xmlns: EPP_XMLNS.to_string(),
            xmlns_xsi: EPP_XMLNS_XSI.to_string(),
            xsi_schema_location: EPP_XSI_SCHEMA_LOCATION.to_string(),
        }
    }
}
