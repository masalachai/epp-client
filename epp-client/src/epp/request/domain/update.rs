use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::{AuthInfo, DomainContact, DomainStatus, HostAttrList, HostObjList};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainUpdate = EppObject<Command<DomainUpdate<HostObjList>>>;
pub type EppDomainUpdateWithHostAttr = EppObject<Command<DomainUpdate<HostAttrList>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainChangeInfo {
    pub registrant: Option<StringValue>,
    #[serde(rename = "authInfo")]
    pub auth_info: Option<AuthInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAddRemove<T> {
    #[serde(rename = "ns")]
    pub ns: Option<T>,
    #[serde(rename = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    #[serde(rename = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainUpdateData<T> {
    xmlns: String,
    name: StringValue,
    add: Option<DomainAddRemove<T>>,
    #[serde(rename = "rem")]
    remove: Option<DomainAddRemove<T>>,
    #[serde(rename = "chg")]
    change_info: Option<DomainChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
pub struct DomainUpdate<T> {
    #[serde(rename = "update")]
    domain: DomainUpdateData<T>,
}

impl EppDomainUpdate {
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

    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    pub fn add(&mut self, add: DomainAddRemove<HostObjList>) {
        self.data.command.domain.add = Some(add);
    }

    pub fn remove(&mut self, remove: DomainAddRemove<HostObjList>) {
        self.data.command.domain.remove = Some(remove);
    }
}

impl EppDomainUpdateWithHostAttr {
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

    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    pub fn add(&mut self, add: DomainAddRemove<HostAttrList>) {
        self.data.command.domain.add = Some(add);
    }

    pub fn remove(&mut self, remove: DomainAddRemove<HostAttrList>) {
        self.data.command.domain.remove = Some(remove);
    }
}
