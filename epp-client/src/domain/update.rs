//! Types for EPP domain check request
//!
use crate::{
    common::{DomainAuthInfo, DomainContact, DomainStatus, HostList, NoExtension, StringValue},
    request::{Command, Transaction},
};

use super::XMLNS;

use serde::Serialize;

impl Transaction<NoExtension> for DomainUpdate {}

impl Command for DomainUpdate {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl DomainUpdate {
    pub fn new(name: &str) -> Self {
        Self {
            domain: DomainUpdateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove) {
        self.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove) {
        self.domain.remove = Some(remove);
    }
}

/// Type for elements under the &lt;chg&gt; tag for domain update
#[derive(Serialize, Debug)]
pub struct DomainChangeInfo {
    /// The new registrant contact for the domain
    #[serde(rename = "domain:registrant", alias = "update")]
    pub registrant: Option<StringValue>,
    /// The new auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    pub auth_info: Option<DomainAuthInfo>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Serialize, Debug)]
pub struct DomainAddRemove {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    pub ns: Option<HostList>,
    /// The list of contacts to add to or remove from the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of statuses to add to or remove from the domain
    #[serde(rename = "domain:status", alias = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
}

/// Type for elements under the &lt;update&gt; tag for domain update
#[derive(Serialize, Debug)]
pub struct DomainUpdateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    pub xmlns: String,
    /// The name of the domain to update
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    #[serde(rename = "domain:add", alias = "add")]
    pub add: Option<DomainAddRemove>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    #[serde(rename = "domain:rem", alias = "rem")]
    pub remove: Option<DomainAddRemove>,
    /// The data under the &lt;chg&gt; tag for domain update
    #[serde(rename = "domain:chg", alias = "chg")]
    pub change_info: Option<DomainChangeInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;update&gt; command for domains
pub struct DomainUpdate {
    #[serde(rename = "domain:update", alias = "update")]
    pub domain: DomainUpdateRequestData,
}
