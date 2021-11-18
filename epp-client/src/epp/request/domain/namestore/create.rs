//! Types for EPP NameStore domain create

use crate::epp::object::data::{
    AuthInfo, DomainContact, HostAttr, HostAttrList, HostList, HostObjList, Period,
};
use crate::epp::object::{EppObject, StringValue, StringValueTrait};
use crate::epp::request::domain::create::{DomainCreate, DomainCreateData};
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};

use super::object::NameStore;

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
/// use epp_client::epp::{EppNamestoreDomainCreate, EppNamestoreDomainCreateResponse};
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
///     // Create an EppNamestoreDomainCreate instance
///     let domain_create = EppNamestoreDomainCreate::new(
///         "eppdev-100.com",
///         1,
///         "eppdev-contact-2",
///         "epP4uthd#v",
///         contacts,
///         generate_client_tr_id(&client).as_str(),
///         "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainCreateResponse
///     let response = client.transact::<_, EppNamestoreDomainCreateResponse>(&domain_create).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppNamestoreDomainCreate = EppObject<CommandWithExtension<DomainCreate, NameStore>>;

impl EppNamestoreDomainCreate {
    /// Creates a new EppObject for NameStore domain create with namestore extension
    pub fn new(
        name: &str,
        period: u16,
        registrant_id: &str,
        auth_password: &str,
        contacts: Vec<DomainContact>,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainCreate {
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

        let command = CommandWithExtension::<DomainCreate, NameStore> {
            command: domain_create,
            extension: Some(Extension {
                data: NameStore {
                    xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_string(),
                    subproduct: subproduct.to_string_value(),
                },
            }),
            client_tr_id: client_tr_id.to_string_value(),
        };

        EppObject::build(command)
    }

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
        subproduct: &str,
    ) -> EppNamestoreDomainCreate {
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

        let command = CommandWithExtension::<DomainCreate, NameStore> {
            command: domain_create,
            extension: Some(Extension {
                data: NameStore {
                    xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_string(),
                    subproduct: subproduct.to_string_value(),
                },
            }),
            client_tr_id: client_tr_id.to_string_value(),
        };

        EppObject::build(command)
    }

    /// Creates a new EppObject for domain create corresponding to the &lt;epp&gt; tag in EPP XML
    /// without any contacts
    pub fn new_without_contacts(
        name: &str,
        period: u16,
        auth_password: &str,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainCreate {
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

        let command = CommandWithExtension::<DomainCreate, NameStore> {
            command: domain_create,
            extension: Some(Extension {
                data: NameStore {
                    xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_string(),
                    subproduct: subproduct.to_string_value(),
                },
            }),
            client_tr_id: client_tr_id.to_string_value(),
        };

        EppObject::build(command)
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
        subproduct: &str,
    ) -> EppNamestoreDomainCreate {
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

        let command = CommandWithExtension::<DomainCreate, NameStore> {
            command: domain_create,
            extension: Some(Extension {
                data: NameStore {
                    xmlns: EPP_DOMAIN_NAMESTORE_EXT_XMLNS.to_string(),
                    subproduct: subproduct.to_string_value(),
                },
            }),
            client_tr_id: client_tr_id.to_string_value(),
        };

        EppObject::build(command)
    }
}
