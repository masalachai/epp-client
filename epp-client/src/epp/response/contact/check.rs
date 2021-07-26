//! Types for EPP contact check response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML contact check response
pub type EppContactCheckResponse = EppObject<CommandResponse<ContactCheckResult>>;

/// Type that represents the &lt;id&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheck {
    /// The text of the &lt;id&gt; tag
    #[serde(rename = "$value")]
    pub id: StringValue,
    /// The avail attr on the &lt;id&gt; tag
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckDataItem {
    /// Data under the &lt;id&gt; tag
    #[serde(rename = "id")]
    pub contact: ContactCheck,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// XML schema location for contact response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub contact_list: Vec<ContactCheckDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for contact check response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCheckResult {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: ContactCheckData,
}
