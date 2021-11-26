//! Types for EPP domain transfer request

use epp_client_macros::*;

use crate::epp::object::data::{DomainAuthInfo, Period};
use crate::epp::object::{ElementName, EppObject, StringValue};
use crate::epp::request::Command;
use crate::epp::response::{CommandResponse, EppCommandResponse};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for transfer request for domain
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::{EppDomainTransferRequest, EppDomainTransferRequestResponse};
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
///     // Create an EppDomainTransferRequest instance
///     let domain_transfer_request = EppDomainTransferRequest::request(
///         "eppdev-100.net", 1, "epP4uthd#v", generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainTransferRequestResponse
///     let response = client.transact::<_, EppDomainTransferRequestResponse>(&domain_transfer_request).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainTransferRequest = EppObject<Command<DomainTransferRequest>>;

/// Type that represents the &lt;epp&gt; request for transfer approval for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::{EppDomainTransferApprove, EppDomainTransferApproveResponse};
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
///     // Create an EppDomainTransferApprove instance
///     let domain_transfer_approve = EppDomainTransferApprove::approve(
///         "eppdev-100.net", generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainTransferApproveResponse
///     let response = client.transact::<_, EppDomainTransferApproveResponse>(&domain_transfer_approve).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainTransferApprove = EppObject<Command<DomainTransferRequest>>;

/// Type that represents the &lt;epp&gt; request for transfer rejection for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::{EppDomainTransferReject, EppDomainTransferRejectResponse};
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
///     // Create an EppDomainTransferReject instance
///     let domain_transfer_reject = EppDomainTransferReject::reject(
///         "eppdev-100.net", generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainTransferRejectResponse
///     let response = client.transact::<_, EppDomainTransferRejectResponse>(&domain_transfer_reject).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainTransferReject = EppObject<Command<DomainTransferRequest>>;

/// Type that represents the &lt;epp&gt; request for transfer request cancellation for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::{EppDomainTransferCancel, EppDomainTransferCancelResponse};
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
///     // Create an EppDomainTransferCancel instance
///     let domain_transfer_cancel = EppDomainTransferCancel::cancel(
///         "eppdev-100.net", generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainTransferCancelResponse
///     let response = client.transact::<_, EppDomainTransferCancelResponse>(&domain_transfer_cancel).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainTransferCancel = EppObject<Command<DomainTransferRequest>>;

/// Type that represents the &lt;epp&gt; request for transfer request query for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::{EppDomainTransferQuery, EppDomainTransferQueryResponse};
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
///     // Create an EppDomainTransferQuery instance
///     let domain_transfer_query = EppDomainTransferQuery::query(
///         "eppdev-100.net", "epP4uthd#v", generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainTransferQueryResponse
///     let response = client.transact::<_, EppDomainTransferQueryResponse>(&domain_transfer_query).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainTransferQuery = EppObject<Command<DomainTransferRequest>>;

impl EppDomainTransferRequest {
    /// Creates a new EppObject for domain transfer request corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn request(
        name: &str,
        years: u16,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppDomainTransferRequest {
        EppObject::build(Command::<DomainTransferRequest>::new(
            DomainTransferRequest {
                operation: "request".to_string(),
                domain: DomainTransferRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    period: Some(Period::new(years)),
                    auth_info: Some(DomainAuthInfo::new(auth_password)),
                },
            },
            client_tr_id,
        ))
    }

    /// Sets the period for renewal in case of a successful transfer
    pub fn set_period(&mut self, period: Period) {
        self.data.command.domain.period = Some(period);
    }
}

impl EppDomainTransferApprove {
    /// Creates a new EppObject for domain transfer approval corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn approve(name: &str, client_tr_id: &str) -> EppDomainTransferApprove {
        EppObject::build(Command::<DomainTransferRequest>::new(
            DomainTransferRequest {
                operation: "approve".to_string(),
                domain: DomainTransferRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: None,
                },
            },
            client_tr_id,
        ))
    }
}

impl EppDomainTransferCancel {
    /// Creates a new EppObject for domain transfer request cancellation corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn cancel(name: &str, client_tr_id: &str) -> EppDomainTransferCancel {
        EppObject::build(Command::<DomainTransferRequest>::new(
            DomainTransferRequest {
                operation: "cancel".to_string(),
                domain: DomainTransferRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: None,
                },
            },
            client_tr_id,
        ))
    }
}

impl EppDomainTransferReject {
    /// Creates a new EppObject for domain transfer rejection corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn reject(name: &str, client_tr_id: &str) -> EppDomainTransferReject {
        EppObject::build(Command::<DomainTransferRequest>::new(
            DomainTransferRequest {
                operation: "reject".to_string(),
                domain: DomainTransferRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: None,
                },
            },
            client_tr_id,
        ))
    }
}

impl EppDomainTransferQuery {
    /// Creates a new EppObject for domain transfer request query corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn query(name: &str, auth_password: &str, client_tr_id: &str) -> EppDomainTransferQuery {
        EppObject::build(Command::<DomainTransferRequest>::new(
            DomainTransferRequest {
                operation: "query".to_string(),
                domain: DomainTransferRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: Some(DomainAuthInfo::new(auth_password)),
                },
            },
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer request response
pub type EppDomainTransferRequestResponse = EppObject<CommandResponse<DomainTransferResponse>>;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer approval response
pub type EppDomainTransferApproveResponse = EppCommandResponse;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer rejection response
pub type EppDomainTransferRejectResponse = EppCommandResponse;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer cancellation response
pub type EppDomainTransferCancelResponse = EppCommandResponse;
/// Type that represents the &lt;epp&gt; tag for the EPP XML domain transfer query response
pub type EppDomainTransferQueryResponse = EppObject<CommandResponse<DomainTransferResponse>>;

// Request

/// Type for elements under the domain &lt;transfer&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The name of the domain under transfer
    #[serde(rename = "domain:name")]
    name: StringValue,
    /// The period of renewal upon a successful transfer
    /// Only applicable in case of a transfer request
    #[serde(rename = "domain:period")]
    period: Option<Period>,
    /// The authInfo for the domain under transfer
    /// Only applicable to domain transfer and domain transfer query requests
    #[serde(rename = "domain:authInfo")]
    auth_info: Option<DomainAuthInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "transfer")]
/// Type for EPP XML &lt;transfer&gt; command for domains
pub struct DomainTransferRequest {
    /// The transfer operation to perform indicated by the 'op' attr
    /// The values are one of transfer, approve, reject, cancel, or query
    #[serde(rename = "op")]
    operation: String,
    /// The data under the &lt;transfer&gt; tag in the transfer request
    #[serde(rename = "domain:transfer")]
    domain: DomainTransferRequestData,
}

// Response

/// Type that represents the &lt;trnData&gt; tag for domain transfer response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The domain name
    pub name: StringValue,
    /// The domain transfer status
    #[serde(rename = "trStatus")]
    pub transfer_status: StringValue,
    /// The epp user who requested the transfer
    #[serde(rename = "reID")]
    pub requester_id: StringValue,
    /// The transfer rquest date
    #[serde(rename = "reDate")]
    pub requested_at: StringValue,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "acID")]
    pub ack_id: StringValue,
    /// THe date by which the acknowledgment should be made
    #[serde(rename = "acDate")]
    pub ack_by: StringValue,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain transfer response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "trnData")]
    pub transfer_data: DomainTransferResponseData,
}
