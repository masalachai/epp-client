//! Types for EPP domain check response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the <epp> tag for the EPP XML domain check response
pub type EppDomainCheckResponse = EppObject<CommandResponse<DomainCheckResult>>;

/// Type that represents the <name> tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheck {
    /// The domain name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The domain (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the <cd> tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckDataItem {
    /// Data under the <name> tag
    #[serde(rename = "name")]
    pub domain: DomainCheck,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the <chkData> tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// XML schema location for domain response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// Data under the <cd> tag
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckDataItem>,
}

/// Type that represents the <resData> tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResult {
    /// Data under the <chkData> tag
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckData,
}
