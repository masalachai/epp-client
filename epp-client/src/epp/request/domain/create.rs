//! Types for EPP domain create request

use epp_client_macros::*;

use crate::epp::object::data::{
    AuthInfo, DomainContact, HostAttr, HostAttrList, HostObjList, Period,
};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;create&gt; command
/// with &lt;hostObj&gt; elements in the request for &lt;ns&gt; list
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::object::data::DomainContact;
/// use epp_client::epp::{EppDomainCreate, EppDomainCreateResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     /// Create a vector of existing domain contact IDs
///     let contacts = vec![
///         DomainContact {
///             contact_type: "admin".to_string(),
///             id: "eppdev-contact-2".to_string()
///         },
///         DomainContact {
///             contact_type: "tech".to_string(),
///             id: "eppdev-contact-2".to_string()
///         },
///         DomainContact {
///             contact_type: "billing".to_string(),
///             id: "eppdev-contact-2".to_string()
///         }
///     ];
///
///     // Create an EppDomainCreate instance
///     let domain_create = EppDomainCreate::new(
///         "eppdev-100.com", 1, "eppdev-contact-2", "epP4uthd#v", contacts, generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainCreateResponse
///     let response = client.transact::<_, EppDomainCreateResponse>(&domain_create).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppDomainCreate = EppObject<Command<DomainCreate>>;

/// Enum that can accept one type which corresponds to either the &lt;hostObj&gt; or &lt;hostAttr&gt;
/// list of tags
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HostList {
    HostObjList(HostObjList),
    HostAttrList(HostAttrList),
}

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateData {
    /// XML namespace for domain commands
    xmlns: String,
    /// The domain name
    name: StringValue,
    /// The period of registration
    period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    ns: Option<HostList>,
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
/// Type for EPP XML &lt;create&gt; command for domains
pub struct DomainCreate {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "create")]
    domain: DomainCreateData,
}

impl EppDomainCreate {
    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// with the &lt;ns&gt; tag containing &lt;hostObj&gt; tags
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

        let domain_create = DomainCreate {
            domain: DomainCreateData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: Period::new(period),
                ns: Some(HostList::HostObjList(HostObjList { hosts: ns_list })),
                registrant: Some(registrant_id.to_string_value()),
                auth_info: AuthInfo::new(auth_password),
                contacts: Some(contacts),
            },
        };

        EppObject::build(Command::<DomainCreate>::new(domain_create, client_tr_id))
    }

    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// without any nameservers
    pub fn new(
        name: &str,
        period: u16,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        let domain_create = DomainCreate {
            domain: DomainCreateData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: Period::new(period),
                ns: None,
                registrant: Some(registrant_id.to_string_value()),
                auth_info: AuthInfo::new(auth_password),
                contacts: Some(contacts),
            },
        };
        EppObject::build(Command::<DomainCreate>::new(domain_create, client_tr_id))
    }

    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// without any contacts
    pub fn new_without_contacts(
        name: &str,
        period: u16,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        let domain_create = DomainCreate {
            domain: DomainCreateData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: Period::new(period),
                ns: None,
                registrant: None,
                auth_info: AuthInfo::new(auth_password),
                contacts: None,
            },
        };

        EppObject::build(Command::<DomainCreate>::new(domain_create, client_tr_id))
    }

    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// with the &lt;ns&gt; tag containing &lt;hostAttr&gt; tags
    pub fn new_with_host_attr(
        name: &str,
        period: u16,
        ns: Vec<HostAttr>,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        let domain_create = DomainCreate {
            domain: DomainCreateData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: Period::new(period),
                ns: Some(HostList::HostAttrList(HostAttrList { hosts: ns })),
                registrant: Some(registrant_id.to_string_value()),
                auth_info: AuthInfo::new(auth_password),
                contacts: Some(contacts),
            },
        };
        EppObject::build(Command::<DomainCreate>::new(domain_create, client_tr_id))
    }
}
