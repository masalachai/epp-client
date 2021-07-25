//! Types for EPP domain create request

use epp_client_macros::*;

use crate::epp::object::data::{
    AuthInfo, DomainContact, HostAttr, HostAttrList, HostObjList, Period,
};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for domain <create> command
/// with <hostObj> elements in the request for <ns> list
pub type EppDomainCreate = EppObject<Command<DomainCreate<HostObjList>>>;
/// Type that represents the <epp> request for domain <create> command
/// with <hostAttr> elements in the request for <ns> list
pub type EppDomainCreateWithHostAttr = EppObject<Command<DomainCreate<HostAttrList>>>;

/// Type for elements under the domain <create> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateData<T> {
    /// XML namespace for domain commands
    xmlns: String,
    /// The domain name
    name: StringValue,
    /// The period of registration
    period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    ns: Option<T>,
    /// The domain registrant
    registrant: Option<StringValue>,
    /// The list of contacts for the domain
    #[serde(rename = "contact")]
    contacts: Option<Vec<DomainContact>>,
    /// The auth info for the domain
    #[serde(rename = "authInfo")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML <create> command for domains
pub struct DomainCreate<T> {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "create")]
    domain: DomainCreateData<T>,
}

impl EppDomainCreate {
    /// Creates a new EppObject for domain create corresponding to the <epp> tag in EPP XML
    /// with the <ns> tag containing <hostObj> tags
    pub fn new_with_ns(
        name: &str,
        period: u16,
        ns: Vec<&str>,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        let ns_list = ns
            .iter()
            .map(|n| n.to_string_value())
            .collect::<Vec<StringValue>>();

        EppObject::build(Command::<DomainCreate<HostObjList>> {
            command: DomainCreate {
                domain: DomainCreateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: Some(HostObjList { hosts: ns_list }),
                    registrant: Some(registrant_id.to_string_value()),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: Some(contacts),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Creates a new EppObject for domain create corresponding to the <epp> tag in EPP XML
    /// without any nameservers
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
                domain: DomainCreateData {
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

    /// Creates a new EppObject for domain create corresponding to the <epp> tag in EPP XML
    /// without any contacts
    pub fn new_without_contacts(
        name: &str,
        period: u16,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        EppObject::build(Command::<DomainCreate<HostObjList>> {
            command: DomainCreate {
                domain: DomainCreateData {
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

    /// Creates a new EppObject for domain create corresponding to the <epp> tag in EPP XML
    /// with the <ns> tag containing <hostAttr> tags
    pub fn new_with_host_attr(
        name: &str,
        period: u16,
        ns: Vec<HostAttr>,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreateWithHostAttr {
        EppObject::build(Command::<DomainCreate<HostAttrList>> {
            command: DomainCreate {
                domain: DomainCreateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    period: Period::new(period),
                    ns: Some(HostAttrList { hosts: ns }),
                    registrant: Some(registrant_id.to_string_value()),
                    auth_info: AuthInfo::new(auth_password),
                    contacts: Some(contacts),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
