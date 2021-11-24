use epp_client_macros::*;

use serde::{Deserialize, Serialize};

use crate::epp::object::{ElementName, EmptyTag, StringValue, StringValueTrait};
use crate::epp::request::EppRequest;
use crate::epp::response::domain::check::EppDomainCheckResponse;
use crate::epp::xml::EPP_DOMAIN_XMLNS;

#[derive(Debug)]
pub struct Check {
    request: Request,
}

impl EppRequest for Check {
    type Extension = EmptyTag;
    type Input = Request;
    type Output = EppDomainCheckResponse;

    fn into_parts(self) -> (Self::Input, Option<Self::Extension>) {
        (self.request, None)
    }
}

impl Check {
    pub fn new(domains: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self {
            request: Request {
                list: DomainList {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domains: domains
                        .into_iter()
                        .map(|d| d.as_ref().to_string_value())
                        .collect::<Vec<StringValue>>(),
                },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct Request {
    /// The object holding the list of domains to be checked
    #[serde(rename = "check")]
    list: DomainList,
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
