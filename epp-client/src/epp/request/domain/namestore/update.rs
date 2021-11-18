//! Types for EPP NameStore domain update with namestore extension

use crate::epp::object::{EppObject, StringValueTrait};
use crate::epp::request::domain::update::{
    DomainAddRemove, DomainChangeInfo, DomainUpdate, DomainUpdateData,
};
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};

use super::object::NameStore;

/// Type that represents the &lt;epp&gt; request for domain &lt;update&gt; command
/// with &lt;hostObj&gt; elements in the request for &lt;ns&gt; list
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::object::data::{DomainStatus, DomainContact};
/// use epp_client::epp::{EppNamestoreDomainUpdate, EppNamestoreDomainUpdateResponse, DomainAddRemove};
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
///     // Create an EppNamestoreDomainUpdate instance
///     let mut domain_update = EppNamestoreDomainUpdate::new("eppdev-100.com", generate_client_tr_id(&client).as_str(), "com");
///
///     let add = DomainAddRemove {
///         ns: None,
///         contacts: None,
///         statuses: Some(vec![
///             DomainStatus {
///                 status: "clientUpdateProhibited".to_string()
///             }
///         ])
///     };
///
///     let remove = DomainAddRemove {
///         ns: None,
///         contacts: Some(vec![
///             DomainContact {
///                 contact_type: "billing".to_string(),
///                 id: "eppdev-contact-2".to_string()
///             }
///         ]),
///         statuses: None,
///     };
///
///     domain_update.add(add);
///     domain_update.remove(remove);
///
///     // send it to the registry and receive a response of type EppNamestoreDomainUpdateResponse
///     let response = client.transact::<_, EppNamestoreDomainUpdateResponse>(&domain_update).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppNamestoreDomainUpdate = EppObject<CommandWithExtension<DomainUpdate, NameStore>>;

impl EppNamestoreDomainUpdate {
    /// Creates a new EppObject for NameStore domain update with namestore extension
    pub fn new(name: &str, client_tr_id: &str, subproduct: &str) -> EppNamestoreDomainUpdate {
        let domain_update = DomainUpdate {
            domain: DomainUpdateData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                add: None,
                remove: None,
                change_info: None,
            },
        };

        let command = CommandWithExtension::<DomainUpdate, NameStore> {
            command: domain_update,
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

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.data.command.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove) {
        self.data.command.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove) {
        self.data.command.domain.remove = Some(remove);
    }
}
