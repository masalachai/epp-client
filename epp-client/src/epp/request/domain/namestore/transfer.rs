//! Types for EPP domain transfer request with namestore extension

use crate::epp::object::data::{AuthInfo, Period};
use crate::epp::object::{EppObject, Extension, StringValueTrait};
use crate::epp::request::domain::transfer::{DomainTransfer, DomainTransferData};
use crate::epp::request::CommandWithExtension;
use crate::epp::xml::{EPP_DOMAIN_NAMESTORE_EXT_XMLNS, EPP_DOMAIN_XMLNS};

use super::object::NameStore;

/// Type that represents the &lt;epp&gt; request for transfer request with namestore extension for domain
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainTransferRequest, EppNamestoreDomainTransferRequestResponse};
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
///     // Create an EppNamestoreDomainTransferRequest instance
///     let domain_transfer_request = EppNamestoreDomainTransferRequest::request(
///         "eppdev-100.net", 1, "epP4uthd#v", generate_client_tr_id(&client).as_str(), "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainTransferRequestResponse
///     let response = client.transact::<_, EppNamestoreDomainTransferRequestResponse>(&domain_transfer_request).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppNamestoreDomainTransferRequest =
    EppObject<CommandWithExtension<DomainTransfer, NameStore>>;

/// Type that represents the &lt;epp&gt; request for transfer approval for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainTransferApprove, EppNamestoreDomainTransferApproveResponse};
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
///     // Create an EppNamestoreDomainTransferApprove instance
///     let domain_transfer_approve = EppNamestoreDomainTransferApprove::approve(
///         "eppdev-100.net", generate_client_tr_id(&client).as_str(), "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainTransferApproveResponse
///     let response = client.transact::<_, EppNamestoreDomainTransferApproveResponse>(&domain_transfer_approve).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppNamestoreDomainTransferApprove =
    EppObject<CommandWithExtension<DomainTransfer, NameStore>>;

/// Type that represents the &lt;epp&gt; request for transfer rejection for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainTransferReject, EppNamestoreDomainTransferRejectResponse};
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
///     // Create an EppNamestoreDomainTransferReject instance
///     let domain_transfer_reject = EppNamestoreDomainTransferReject::reject(
///         "eppdev-100.net", generate_client_tr_id(&client).as_str(), "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainTransferRejectResponse
///     let response = client.transact::<_, EppNamestoreDomainTransferRejectResponse>(&domain_transfer_reject).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppNamestoreDomainTransferReject =
    EppObject<CommandWithExtension<DomainTransfer, NameStore>>;

/// Type that represents the &lt;epp&gt; request for transfer request cancellation for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainTransferCancel, EppNamestoreDomainTransferCancelResponse};
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
///     // Create an EppNamestoreDomainTransferCancel instance
///     let domain_transfer_cancel = EppNamestoreDomainTransferCancel::cancel(
///         "eppdev-100.net", generate_client_tr_id(&client).as_str(), "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainTransferCancelResponse
///     let response = client.transact::<_, EppNamestoreDomainTransferCancelResponse>(&domain_transfer_cancel).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppNamestoreDomainTransferCancel =
    EppObject<CommandWithExtension<DomainTransfer, NameStore>>;

/// Type that represents the &lt;epp&gt; request for transfer request query for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNamestoreDomainTransferQuery, EppNamestoreDomainTransferQueryResponse};
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
///     // Create an EppNamestoreDomainTransferQuery instance
///     let domain_transfer_query = EppNamestoreDomainTransferQuery::query(
///         "eppdev-100.net", "epP4uthd#v", generate_client_tr_id(&client).as_str(), "com"
///     );
///
///     // send it to the registry and receive a response of type EppNamestoreDomainTransferQueryResponse
///     let response = client.transact::<_, EppNamestoreDomainTransferQueryResponse>(&domain_transfer_query).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppNamestoreDomainTransferQuery =
    EppObject<CommandWithExtension<DomainTransfer, NameStore>>;

impl EppNamestoreDomainTransferRequest {
    /// Creates a new EppObject for domain transfer request corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn request(
        name: &str,
        years: u16,
        auth_password: &str,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainTransferRequest {
        let domain_transfer = DomainTransfer {
            operation: "request".to_string(),
            domain: DomainTransferData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: Some(Period::new(years)),
                auth_info: Some(AuthInfo::new(auth_password)),
            },
        };

        let command = CommandWithExtension::<DomainTransfer, NameStore> {
            command: domain_transfer,
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

    /// Sets the period for renewal in case of a successful transfer
    pub fn set_period(&mut self, period: Period) {
        self.data.command.domain.period = Some(period);
    }
}

impl EppNamestoreDomainTransferApprove {
    /// Creates a new EppObject for domain transfer approval corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn approve(
        name: &str,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainTransferApprove {
        let domain_transfer = DomainTransfer {
            operation: "approve".to_string(),
            domain: DomainTransferData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: None,
                auth_info: None,
            },
        };

        let command = CommandWithExtension::<DomainTransfer, NameStore> {
            command: domain_transfer,
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

impl EppNamestoreDomainTransferCancel {
    /// Creates a new EppObject for domain transfer request cancellation corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn cancel(
        name: &str,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainTransferCancel {
        let domain_transfer = DomainTransfer {
            operation: "cancel".to_string(),
            domain: DomainTransferData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: None,
                auth_info: None,
            },
        };

        let command = CommandWithExtension::<DomainTransfer, NameStore> {
            command: domain_transfer,
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

impl EppNamestoreDomainTransferReject {
    /// Creates a new EppObject for domain transfer rejection corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn reject(
        name: &str,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainTransferReject {
        let domain_transfer = DomainTransfer {
            operation: "reject".to_string(),
            domain: DomainTransferData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: None,
                auth_info: None,
            },
        };

        let command = CommandWithExtension::<DomainTransfer, NameStore> {
            command: domain_transfer,
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

impl EppNamestoreDomainTransferQuery {
    /// Creates a new EppObject for domain transfer request query corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn query(
        name: &str,
        auth_password: &str,
        client_tr_id: &str,
        subproduct: &str,
    ) -> EppNamestoreDomainTransferQuery {
        let domain_transfer = DomainTransfer {
            operation: "query".to_string(),
            domain: DomainTransferData {
                xmlns: EPP_DOMAIN_XMLNS.to_string(),
                name: name.to_string_value(),
                period: None,
                auth_info: Some(AuthInfo::new(auth_password)),
            },
        };

        let command = CommandWithExtension::<DomainTransfer, NameStore> {
            command: domain_transfer,
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
