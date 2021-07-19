use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct EppObject<T> {
    pub xmlns: String,
    #[serde(rename = "xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "xsi:schemaLocation")]
    pub xsi_schema_location: String,
    pub data: T,
}
