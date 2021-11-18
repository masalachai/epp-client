//! Types for EPP domain renew response
use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain renew response
pub type EppDomainRenewResponse = EppObject<CommandResponse<DomainRenewResult>>;

/// Type that represents the &lt;renData&gt; tag for domain renew response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The name of the domain
    pub name: StringValue,
    /// The new expiry date after renewal
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain renew response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewResult {
    /// Data under the &lt;renData&gt; tag
    #[serde(rename = "renData")]
    pub renew_data: DomainRenewData,
}
