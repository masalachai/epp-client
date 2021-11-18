use epp_client_macros::*;

use crate::epp::object::{ElementName, EmptyTag, EppObject};
use crate::epp::response::CommandResponseWithExtension;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; tag for the EPP XML rgp restore request response
pub type EppDomainRgpRestoreRequestResponse =
    EppObject<CommandResponseWithExtension<EmptyTag, RgpRequestResult>>;

/// Type that represents the &lt;rgpStatus&gt; tag for domain rgp restore request response
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpStatus {
    /// The domain RGP status
    #[serde(rename = "s")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[serde(rename = "upData")]
#[element_name(name = "upData")]
/// Type that represents the &lt;resData&gt; tag for domain transfer response
pub struct RgpRequestResult {
    #[serde(rename = "xmlns:rgp")]
    xmlns: String,
    /// Data under the &lt;rgpStatus&gt; tag
    #[serde(rename = "rgpStatus")]
    pub rgp_status: RgpStatus,
}
