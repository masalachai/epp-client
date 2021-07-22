use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::{AuthInfo, Period};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppDomainCreate<T> = EppObject<Command<DomainCreate<T>>>;

pub enum HostType {
    HostObj,
    HostAttr,
}

pub trait HostList {
    fn new(ns: Vec<&str>) -> Self;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainContact {
    #[serde(rename = "$value")]
    pub id: String,
    #[serde(rename = "type")]
    pub contact_type: String,
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
    registrant: StringValue,
    #[serde(rename = "contact")]
    contacts: Vec<DomainContact>,
    #[serde(rename = "authInfo")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
pub struct DomainCreate<T> {
    #[serde(rename = "create")]
    domain: DomainData<T>,
}

impl<T: HostList> EppDomainCreate<T> {
    pub fn new(
        name: &str,
        period: u16,
        ns: Vec<&str>,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate<T> {
        EppObject::build(Command::<DomainCreate<T>> {
            command: DomainCreate {
                domain: DomainData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: Some(T::new(ns)),
                    registrant: registrant_id.to_string_value(),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: contacts,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn new_without_ns(
        name: &str,
        period: u16,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate<T> {
        EppObject::build(Command::<DomainCreate<T>> {
            command: DomainCreate {
                domain: DomainData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: None,
                    registrant: registrant_id.to_string_value(),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: contacts,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
