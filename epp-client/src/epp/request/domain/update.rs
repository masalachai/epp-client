//! Types for EPP domain check request

use epp_client_macros::*;

use crate::epp::object::data::{AuthInfo, DomainContact, DomainStatus, HostAttrList, HostObjList};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for domain <update> command
/// with <hostObj> elements in the request for <ns> list
pub type EppDomainUpdate = EppObject<Command<DomainUpdate<HostObjList>>>;
/// Type that represents the <epp> request for domain <update> command
/// with <hostAttr> elements in the request for <ns> list
pub type EppDomainUpdateWithHostAttr = EppObject<Command<DomainUpdate<HostAttrList>>>;

/// Type for elements under the <chg> tag for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainChangeInfo {
    /// The new registrant contact for the domain
    pub registrant: Option<StringValue>,
    /// The new auth info for the domain
    #[serde(rename = "authInfo")]
    pub auth_info: Option<AuthInfo>,
}

/// Type for elements under the <add> and <rem> tags for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAddRemove<T> {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    #[serde(rename = "ns")]
    pub ns: Option<T>,
    /// The list of contacts to add to or remove from the domain
    #[serde(rename = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of statuses to add to or remove from the domain
    #[serde(rename = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
}

/// Type for elements under the <update> tag for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainUpdateData<T> {
    /// XML namespace for domain commands
    xmlns: String,
    /// The name of the domain to update
    name: StringValue,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    add: Option<DomainAddRemove<T>>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    #[serde(rename = "rem")]
    remove: Option<DomainAddRemove<T>>,
    /// The data under the <chg> tag for domain update
    #[serde(rename = "chg")]
    change_info: Option<DomainChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML <update> command for domains
pub struct DomainUpdate<T> {
    #[serde(rename = "update")]
    domain: DomainUpdateData<T>,
}

impl EppDomainUpdate {
    /// Creates a new EppObject for domain update corresponding to the <epp> tag in EPP XML
    /// with the <ns> tag containing <hostObj> tags
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainUpdate {
        EppObject::build(Command::<DomainUpdate<HostObjList>> {
            command: DomainUpdate {
                domain: DomainUpdateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the data for the <chg> tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    /// Sets the data for the <add> tag
    pub fn add(&mut self, add: DomainAddRemove<HostObjList>) {
        self.data.command.domain.add = Some(add);
    }

    /// Sets the data for the <rem> tag
    pub fn remove(&mut self, remove: DomainAddRemove<HostObjList>) {
        self.data.command.domain.remove = Some(remove);
    }
}

impl EppDomainUpdateWithHostAttr {
    /// Creates a new EppObject for domain update corresponding to the <epp> tag in EPP XML
    /// with the <ns> tag containing <hostAttr> tags
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainUpdateWithHostAttr {
        EppObject::build(Command::<DomainUpdate<HostAttrList>> {
            command: DomainUpdate {
                domain: DomainUpdateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the data for the <chg> tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    /// Sets the data for the <add> tag
    pub fn add(&mut self, add: DomainAddRemove<HostAttrList>) {
        self.data.command.domain.add = Some(add);
    }

    /// Sets the data for the <rem> tag
    pub fn remove(&mut self, remove: DomainAddRemove<HostAttrList>) {
        self.data.command.domain.remove = Some(remove);
    }
}
