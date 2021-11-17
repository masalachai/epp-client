//! Types for EPP NameStore domain info

use crate::epp::object::{EppObject, StringValueTrait};
use crate::epp::request::domain::info::{Domain, DomainInfo, DomainInfoData};
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};

use super::object::NameStore;

/// Type that represents the &lt;epp&gt; request for domain &lt;info&gt; command with namestore extension
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainInfo, EppNamestoreDomainInfoResponse};
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
///     // Create an EppNamestoreDomainInfo instance
///     let domain_info = EppNamestoreDomainInfo::new("eppdev-100.com", generate_client_tr_id(&client).as_str(), "com");
///
///     // send it to the registry and receive a response of type EppNamestoreDomainInfoResponse
///     let response = client.transact::<_, EppNamestoreDomainInfoResponse>(&domain_info).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppNamestoreDomainInfo = EppObject<CommandWithExtension<DomainInfo, NameStore>>;

impl EppNamestoreDomainInfo {
    /// Creates a new EppObject for NameStore domain check
    pub fn new(name: &str, client_tr_id: &str, subproduct: &str) -> EppNamestoreDomainInfo {
        let domain_info = DomainInfo {
            info: DomainInfoData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                domain: Domain {
                    hosts: "all".to_string(),
                    name: name.to_string(),
                },
            },
        };

        let command = CommandWithExtension::<DomainInfo, NameStore> {
            command: domain_info,
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
