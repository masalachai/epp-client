//! Types for EPP contact create response

use serde::{Deserialize, Serialize};

use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::CommandResponse;

/// Type that represents the &lt;epp&gt; tag for the EPP XML contact create response
pub type EppContactCreateResponse = EppObject<CommandResponse<ContactCreateResult>>;

/// Type that represents the &lt;creData&gt; tag for contact create response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateData {
    /// XML namespace for contact response data
    #[serde(rename = "xmlns:contact")]
    xmlns: String,
    /// XML schema location for contact response data
    #[serde(rename = "xsi:schemaLocation")]
    schema_location: String,
    /// The contact id
    pub id: StringValue,
    #[serde(rename = "crDate")]
    /// The contact creation date
    pub created_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for contact create response
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactCreateResult {
    /// Data under the &lt;creData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: ContactCreateData,
}
