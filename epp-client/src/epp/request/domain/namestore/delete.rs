//! Types for EPP NameStore domain delete

use crate::epp::object::{EppObject, StringValueTrait};
use crate::epp::request::domain::delete::{DomainDelete, DomainDeleteData};
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};

use super::object::NameStore;

/// Type that represents the &lt;epp&gt; request for domain &lt;delete&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainDelete, EppNamestoreDomainDeleteResponse};
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
///     // Create an EppNamestoreDomainDelete instance
///     let mut domain_delete = EppNamestoreDomainDelete::new("eppdev-100.com", generate_client_tr_id(&client).as_str(), "com");
///
///     // send it to the registry and receive a response of type EppNamestoreDomainDeleteResponse
///     let response = client.transact::<_, EppNamestoreDomainDeleteResponse>(&domain_delete).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppNamestoreDomainDelete = EppObject<CommandWithExtension<DomainDelete, NameStore>>;

impl EppNamestoreDomainDelete {
    /// Creates a new EppObject for NameStore domain delete
    pub fn new(name: &str, client_tr_id: &str, subproduct: &str) -> EppNamestoreDomainDelete {
        let domain_delete = DomainDelete {
            domain: DomainDeleteData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
            },
        };

        let command = CommandWithExtension::<DomainDelete, NameStore> {
            command: domain_delete,
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
