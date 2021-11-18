use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::epp::object::{ElementName, StringValue};

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "namestoreExt:namestoreExt")]
pub struct NameStore {
    #[serde(rename = "xmlns:namestoreExt")]
    pub xmlns: String,
    #[serde(rename = "namestoreExt:subProduct")]
    pub subproduct: StringValue,
}
