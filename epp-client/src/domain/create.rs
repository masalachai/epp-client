//! Types for EPP domain create request

use epp_client_macros::*;

use super::EPP_DOMAIN_XMLNS;
use crate::common::{
    DomainAuthInfo, DomainContact, ElementName, EppObject, HostAttr, HostAttrList, HostList,
    HostObjList, Period, StringValue,
};
use crate::request::Command;
use crate::response::CommandResponse;
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
/// use epp_client::common::DomainContact;
/// use epp_client::domain::create::{EppDomainCreate, EppDomainCreateResponse};
/// use epp_client::generate_client_tr_id;
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
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainCreate = EppObject<Command<DomainCreateRequest>>;

impl EppDomainCreate {
    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// with the &lt;ns&gt; tag containing &lt;hostObj&gt; tags
    pub fn new_with_ns(
        name: &str,
        period: u16,
        ns: &[&str],
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        let ns_list = ns.iter().map(|&n| n.into()).collect();

        let domain_create = DomainCreateRequest {
            domain: DomainCreateRequestData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns: Some(HostList::HostObjList(HostObjList { hosts: ns_list })),
                registrant: Some(registrant_id.into()),
                auth_info: DomainAuthInfo::new(auth_password),
                contacts: Some(contacts),
            },
        };

        EppObject::build(Command::<DomainCreateRequest>::new(
            domain_create,
            client_tr_id,
        ))
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
        let domain_create = DomainCreateRequest {
            domain: DomainCreateRequestData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns: None,
                registrant: Some(registrant_id.into()),
                auth_info: DomainAuthInfo::new(auth_password),
                contacts: Some(contacts),
            },
        };
        EppObject::build(Command::<DomainCreateRequest>::new(
            domain_create,
            client_tr_id,
        ))
    }

    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// without any contacts
    pub fn new_without_contacts(
        name: &str,
        period: u16,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppDomainCreate {
        let domain_create = DomainCreateRequest {
            domain: DomainCreateRequestData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns: None,
                registrant: None,
                auth_info: DomainAuthInfo::new(auth_password),
                contacts: None,
            },
        };

        EppObject::build(Command::<DomainCreateRequest>::new(
            domain_create,
            client_tr_id,
        ))
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
        let domain_create = DomainCreateRequest {
            domain: DomainCreateRequestData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns: Some(HostList::HostAttrList(HostAttrList { hosts: ns })),
                registrant: Some(registrant_id.into()),
                auth_info: DomainAuthInfo::new(auth_password),
                contacts: Some(contacts),
            },
        };
        EppObject::build(Command::<DomainCreateRequest>::new(
            domain_create,
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain create response
pub type EppDomainCreateResponse = EppObject<CommandResponse<DomainCreateResponse>>;

// Request

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The domain name
    #[serde(rename = "domain:name", alias = "name")]
    name: StringValue,
    /// The period of registration
    #[serde(rename = "domain:period", alias = "period")]
    period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    ns: Option<HostList>,
    /// The domain registrant
    #[serde(rename = "domain:registrant", alias = "registrant")]
    registrant: Option<StringValue>,
    /// The list of contacts for the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    contacts: Option<Vec<DomainContact>>,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    auth_info: DomainAuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for domains
pub struct DomainCreateRequest {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "domain:create", alias = "create")]
    domain: DomainCreateRequestData,
}

// Response

/// Type that represents the &lt;chkData&gt; tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The domain name
    pub name: StringValue,
    /// The creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: DomainCreateResponseData,
}
