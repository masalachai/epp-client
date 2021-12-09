//! Types for EPP contact delete request

use super::XMLNS;
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::Serialize;

impl Transaction<NoExtension> for ContactDelete {}

impl Command for ContactDelete {
    type Response = ();
    const COMMAND: &'static str = "delete";
}

/// Type containing the data for the &lt;delete&gt; tag for contacts
#[derive(Serialize, Debug)]
pub struct ContactDeleteRequestData {
    /// XML namespace for the &lt;delete&gt; command for contacts
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// The id of the contact to be deleted
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
}

#[derive(Serialize, Debug)]
/// The &lt;delete&gt; type for the contact delete EPP command
pub struct ContactDelete {
    #[serde(rename = "contact:delete", alias = "delete")]
    /// The data for the &lt;delete&gt; tag for a contact delete command
    contact: ContactDeleteRequestData,
}

impl ContactDelete {
    pub fn new(id: &str) -> ContactDelete {
        Self {
            contact: ContactDeleteRequestData {
                xmlns: XMLNS.to_string(),
                id: id.into(),
            },
        }
    }
}
