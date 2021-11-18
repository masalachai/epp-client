use crate::epp::object::ElementName;
use epp_client_macros::ElementName;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[serde(rename = "namestoreExt")]
#[element_name(name = "namestoreExt")]
pub struct EppNamestoreDomainCheckResult {
    #[serde(rename = "xmlns:namestoreExt")]
    xmlns: String,
    #[serde(rename = "subProduct")]
    pub sub_product: String,
}
