//! Types for EPP NameStore domain check

use crate::epp::object::{EppObject, StringValue, StringValueTrait};
use crate::epp::request::domain::check::DomainCheck;
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};
use crate::epp::DomainList;

use super::object::NameStore;

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command with namestore extension
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainCheck, EppNamestoreDomainCheckResponse};
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
///     // Create an EppNamestoreDomainCheck instance
///     let namestore_domain_check = EppNamestoreDomainCheck::new(
///         vec!["eppdev-1.com", "eppdev-2.com"],
///         generate_client_tr_id(&client).as_str(),
///         "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainCheckResponse
///     let response = client.transact::<_, EppNamestoreDomainCheckResponse>(&namestore_domain_check).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppNamestoreDomainCheck = EppObject<CommandWithExtension<DomainCheck, NameStore>>;

impl EppNamestoreDomainCheck {
    /// Creates a new EppObject for NameStore domain check
    pub fn new(
        domains: Vec<&str>,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainCheck {
        let domains = domains
            .iter()
            .map(|d| d.to_string_value())
            .collect::<Vec<StringValue>>();

        let domain_check = DomainCheck {
            list: DomainList {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                domains,
            },
        };

        let command = CommandWithExtension::<DomainCheck, NameStore> {
            command: domain_check,
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
