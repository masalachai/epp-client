//! Types for EPP host create response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the <epp> tag for the EPP XML host create response
pub type EppHostCreateResponse = EppObject<CommandResponse<HostCreateResult>>;

/// Type that represents the <creData> tag for host create response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateData {
    /// XML namespace for host response data
    #[serde(rename = "xmlns:host")]
    xmlns: String,
    /// XML schema location for host response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// The host name
    pub name: StringValue,
    /// The host creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
}

/// Type that represents the <resData> tag for host check response
#[derive(Serialize, Deserialize, Debug)]
pub struct HostCreateResult {
    /// Data under the <creData> tag
    #[serde(rename = "creData")]
    pub create_data: HostCreateData,
}
