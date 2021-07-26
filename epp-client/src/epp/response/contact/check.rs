//! Types for EPP contact check response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the <epp> tag for the EPP XML contact check response
pub type EppContactCheckResponse = EppObject<CommandResponse<ContactCheckResult>>;

/// Type that represents the <id> tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheck {
    /// The text of the <id> tag
    #[serde(rename = "$value")]
    pub id: StringValue,
    /// The avail attr on the <id> tag
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the <cd> tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckDataItem {
    /// Data under the <id> tag
    #[serde(rename = "id")]
    pub contact: ContactCheck,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the <chkData> tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// XML schema location for contact response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// Data under the <cd> tag
    #[serde(rename = "cd")]
    pub contact_list: Vec<ContactCheckDataItem>,
}

/// Type that represents the <resData> tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResult {
    /// Data under the <chkData> tag
    #[serde(rename = "chkData")]
    pub check_data: ContactCheckData,
}
