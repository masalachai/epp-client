//! Types for EPP domain check response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain check response
pub type EppDomainCheckResponse = EppObject<CommandResponse<DomainCheckResult>>;

/// Type that represents the &lt;name&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheck {
    /// The domain name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The domain (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub domain: DomainCheck,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResult {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckData,
}
