//! Data types common to EPP Requests and Responses

pub mod data;

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::fmt::Display;

use crate::epp::xml::{EPP_XMLNS, EPP_XMLNS_XSI, EPP_XSI_SCHEMA_LOCATION};

/// Wraps String for easier serialization to and from values that are inner text
/// for tags rather than attributes
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

/// Trait for StringValue type to add easier conversion from str and String
pub trait StringValueTrait {
    fn to_string_value(&self) -> StringValue;
}

impl StringValueTrait for &str {
    fn to_string_value(&self) -> StringValue {
        StringValue(self.to_string())
    }
}

impl StringValueTrait for String {
    fn to_string_value(&self) -> StringValue {
        StringValue(self.to_string())
    }
}

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait ElementName {
    fn element_name(&self) -> &'static str;
}

/// An EPP XML Document that is used either as an EPP XML request or
/// an EPP XML response
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct EppObject<T: ElementName> {
    /// XML namespace for the &lt;epp&gt; tag
    pub xmlns: String,
    /// Schema namespace for the &lt;epp&gt; tag
    #[serde(rename = "xmlns:xsi")]
    pub xmlns_xsi: String,
    /// Schema location attribute for &lt;epp&gt;
    #[serde(rename = "xsi:schemaLocation")]
    pub xsi_schema_location: String,
    /// the request or response object that is set or received in the EPP XML document
    #[serde(alias = "greeting", alias = "response")]
    pub data: T,
    // TODO: save serialized xml in the instance for debugging or client logging purposes
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

/// The <option> type in EPP XML login requests
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "options")]
pub struct Options {
    /// The EPP version being used
    pub version: StringValue,
    /// The language that will be used during EPP transactions
    pub lang: StringValue,
}

impl Options {
    /// Creates an Options object with version and lang data
    pub fn build(version: &str, lang: &str) -> Options {
        Options {
            version: version.to_string_value(),
            lang: lang.to_string_value(),
        }
    }
}

/// The <svcExtension> type in EPP XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "svcExtension")]
pub struct ServiceExtension {
    /// The service extension URIs being represented by <extURI> in EPP XML
    #[serde(rename = "extURI")]
    pub ext_uris: Option<Vec<StringValue>>,
}

/// The <svcs> type in EPP XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Services {
    /// The service URIs being used by this EPP session represented by <objURI> in EPP XML
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue>,
    /// The <svcExtention> being used in this EPP session
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension>,
}

impl<T: ElementName> EppObject<T> {
    /// Create the enclosing EPP XML tag &lt;epp&gt; for data that represents an EPP XML request or response
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
