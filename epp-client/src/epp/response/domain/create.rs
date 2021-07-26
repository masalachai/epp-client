//! Types for EPP domain create response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the <epp> tag for the EPP XML domain create response
pub type EppDomainCreateResponse = EppObject<CommandResponse<DomainCreateResult>>;

/// Type that represents the <chkData> tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// XML schema location for domain response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// The domain name
    pub name: StringValue,
    /// The creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the <resData> tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResult {
    /// Data under the <chkData> tag
    #[serde(rename = "creData")]
    pub create_data: DomainCreateData,
}
