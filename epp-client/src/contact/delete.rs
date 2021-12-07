//! Types for EPP contact delete request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, StringValue};
use crate::request::{EppExtension, Transaction};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactDelete<E> {
    request: ContactDeleteRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for ContactDelete<E> {
    type Input = ContactDeleteRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> ContactDelete<E> {
    pub fn new(id: &str) -> ContactDelete<NoExtension> {
        ContactDelete {
            request: ContactDeleteRequest {
                contact: ContactDeleteRequestData {
                    xmlns: XMLNS.to_string(),
                    id: id.into(),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactDelete<F> {
        ContactDelete {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDeleteRequestData {
    /// XML namespace for the &lt;delete&gt; command for contacts
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// The id of the contact to be deleted
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "delete")]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDeleteRequest {
    #[serde(rename = "contact:delete", alias = "delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequestData,
}
