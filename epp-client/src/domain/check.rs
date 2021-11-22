use epp_client_macros::*;

use serde::{Deserialize, Serialize};

use crate::epp::object::{ElementName, EmptyTag, StringValue, StringValueTrait};
use crate::epp::request::EppRequest;
use crate::epp::response::domain::check::EppDomainCheckResponse;
use crate::epp::xml::EPP_DOMAIN_XMLNS;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct Request {
    /// The object holding the list of domains to be checked
    #[serde(rename = "check")]
    list: DomainList,
}

impl EppRequest for Request {
    type Output = EppDomainCheckResponse;
    type Extension = EmptyTag;
}

impl Request {
    pub fn new(domains: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self {
            list: DomainList {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                domains: domains
                    .into_iter()
                    .map(|d| d.as_ref().to_string_value())
                    .collect::<Vec<StringValue>>(),
            },
        }
    }
}

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    /// XML namespace for domain commands
    pub xmlns: String,
    #[serde(rename = "name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}
