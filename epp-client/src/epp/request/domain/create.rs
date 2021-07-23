use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::{AuthInfo, DomainContact, Period};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainCreate = EppObject<Command<DomainCreate<HostObjList>>>;
pub type EppDomainCreateWithHostAttr = EppObject<Command<DomainCreate<HostAttrList>>>;

pub enum HostType {
    HostObj,
    HostAttr,
}

pub trait HostList {
    fn new(ns: Vec<&str>) -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttr {
    #[serde(rename = "hostName")]
    host_name: StringValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostAttrList {
    #[serde(rename = "hostAttr")]
    hosts: Vec<HostAttr>,
}

impl HostList for HostAttrList {
    fn new(ns: Vec<&str>) -> HostAttrList {
        let ns_list = ns
            .iter()
            .map(|n| HostAttr {
                host_name: n.to_string_value(),
            })
            .collect::<Vec<HostAttr>>();

        HostAttrList { hosts: ns_list }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HostObjList {
    #[serde(rename = "hostObj")]
    hosts: Vec<StringValue>,
}

impl HostList for HostObjList {
    fn new(ns: Vec<&str>) -> HostObjList {
        let ns_list = ns
            .iter()
            .map(|n| n.to_string_value())
            .collect::<Vec<StringValue>>();

        HostObjList { hosts: ns_list }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainData<T> {
    xmlns: String,
    name: StringValue,
    period: Period,
    ns: Option<T>,
    registrant: Option<StringValue>,
    #[serde(rename = "contact")]
    contacts: Option<Vec<DomainContact>>,
    #[serde(rename = "authInfo")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
pub struct DomainCreate<T> {
    #[serde(rename = "create")]
    domain: DomainData<T>,
}

impl EppDomainCreate {
    pub fn new_with_ns(
        name: &str,
        period: u16,
        ns: Vec<&str>,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        EppObject::build(Command::<DomainCreate<HostObjList>> {
            command: DomainCreate {
                domain: DomainData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: Some(HostObjList::new(ns)),
                    registrant: Some(registrant_id.to_string_value()),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: Some(contacts),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn new(
        name: &str,
        period: u16,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        EppObject::build(Command::<DomainCreate<HostObjList>> {
            command: DomainCreate {
                domain: DomainData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: None,
                    registrant: Some(registrant_id.to_string_value()),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: Some(contacts),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn new_without_contacts(
        name: &str,
        period: u16,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        EppObject::build(Command::<DomainCreate<HostObjList>> {
            command: DomainCreate {
                domain: DomainData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: None,
                    registrant: None,
                    auth_info: AuthInfo::new(auth_password),
                    contacts: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn new_with_host_attr(
        name: &str,
        period: u16,
        ns: Vec<&str>,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreateWithHostAttr {
        EppObject::build(Command::<DomainCreate<HostAttrList>> {
            command: DomainCreate {
                domain: DomainData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: Some(HostAttrList::new(ns)),
                    registrant: Some(registrant_id.to_string_value()),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: Some(contacts),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
