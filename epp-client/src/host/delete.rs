//! Types for EPP host delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, Transaction};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HostDelete<E> {
    request: HostDeleteRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for HostDelete<E> {
    type Input = HostDeleteRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> HostDelete<E> {
    pub fn new(name: &str) -> HostDelete<NoExtension> {
        HostDelete {
            request: HostDeleteRequest {
                host: HostDeleteRequestData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> HostDelete<F> {
        HostDelete {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type for data under the host &lt;delete&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostDeleteRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The host to be deleted
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// Type for EPP XML &lt;delete&gt; command for hosts
pub struct HostDeleteRequest {
    /// The instance holding the data for the host to be deleted
    #[serde(rename = "host:delete", alias = "delete")]
    host: HostDeleteRequestData,
}
