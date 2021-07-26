//! Types for EPP domain renew response
use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the <epp> tag for the EPP XML domain renew response
pub type EppDomainRenewResponse = EppObject<CommandResponse<DomainRenewResult>>;

/// Type that represents the <renData> tag for domain renew response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// XML schema location for domain response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// The name of the domain
    pub name: StringValue,
    /// The new expiry date after renewal
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the <resData> tag for domain renew response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewResult {
    /// Data under the <renData> tag
    #[serde(rename = "renData")]
    pub renew_data: DomainRenewData,
}
