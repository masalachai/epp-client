//! Types for EPP domain delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, Transaction};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainDelete<E> {
    request: DomainDeleteRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for DomainDelete<E> {
    type Input = DomainDeleteRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> DomainDelete<E> {
    pub fn new(name: &str) -> DomainDelete<NoExtension> {
        DomainDelete {
            request: DomainDeleteRequest {
                domain: DomainDeleteRequestData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainDelete<F> {
        DomainDelete {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type for &lt;name&gt; element under the domain &lt;delete&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainDeleteRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The domain to be deleted
    #[serde(rename = "domain:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML &lt;delete&gt; command for domains
pub struct DomainDeleteRequest {
    /// The data under the &lt;delete&gt; tag for domain deletion
    #[serde(rename = "domain:delete", alias = "delete")]
    domain: DomainDeleteRequestData,
}
