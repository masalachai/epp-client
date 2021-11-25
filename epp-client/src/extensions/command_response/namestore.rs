use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::epp::{
    object::{ElementName, StringValue},
    request::EppExtension,
};

const EPP_DOMAIN_NAMESTORE_EXT_XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

impl NameStore {
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_owned(),
            subproduct: subproduct.into(),
        }
    }
}

impl EppExtension for NameStore {
    type Response = NameStore;
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "namestoreExt:namestoreExt")]
pub struct NameStore {
    #[serde(rename = "xmlns:namestoreExt")]
    pub xmlns: String,
    #[serde(rename = "namestoreExt:subProduct")]
    pub subproduct: StringValue,
}
