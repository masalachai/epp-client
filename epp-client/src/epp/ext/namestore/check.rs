use epp_client_macros::*;

use crate::epp::ext::namestore::xml::EPP_DOMAIN_NAMESTORE_EXT_XMLNS;
use crate::epp::object::{ElementName, StringValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "namestoreExt:namestoreExt")]
pub struct NamestoreCheck {
    #[serde(rename = "xmlns:namestoreExt", alias = "xmlns")]
    xmlns: String,
    #[serde(rename = "namestoreExt:subProduct", alias = "subProduct")]
    sub_product: StringValue,
}

impl NamestoreCheck {
    pub fn new(sub_product: &str) -> Self {
        Self {
            xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.into(),
            sub_product: sub_product.into(),
        }
    }
}
