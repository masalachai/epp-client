//! Types for EPP domain transfer request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{DomainAuthInfo, ElementName, NoExtension, Period, StringValue};
use crate::request::{EppExtension, EppRequest};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainTransferRequest<E> {
    request: DomainTransferReq,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainTransferRequest<E> {
    type Input = DomainTransferReq;
    type Output = DomainTransferResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

#[derive(Debug)]
pub struct DomainTransferApprove<E> {
    request: DomainTransferReq,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainTransferApprove<E> {
    type Input = DomainTransferReq;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

#[derive(Debug)]
pub struct DomainTransferReject<E> {
    request: DomainTransferReq,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainTransferReject<E> {
    type Input = DomainTransferReq;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

#[derive(Debug)]
pub struct DomainTransferCancel<E> {
    request: DomainTransferReq,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainTransferCancel<E> {
    type Input = DomainTransferReq;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

#[derive(Debug)]
pub struct DomainTransferQuery<E> {
    request: DomainTransferReq,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainTransferQuery<E> {
    type Input = DomainTransferReq;
    type Output = DomainTransferResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for transfer request for domain
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::DomainTransferRequest;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an DomainTransferRequest instance
///     let domain_transfer_request = DomainTransferRequest::<NoExtension>::new(
///         "eppdev-100.net", 1, "epP4uthd#v"
///     );
///
///     // send it to the registry and receive a response of type DomainTransferRequestResponse
///     let response = client.transact(domain_transfer_request, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainTransferRequest<E> {
    pub fn new(name: &str, years: u16, auth_password: &str) -> DomainTransferRequest<NoExtension> {
        DomainTransferRequest {
            request: DomainTransferReq {
                operation: "request".to_string(),
                domain: DomainTransferReqData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                    period: Some(Period::new(years)),
                    auth_info: Some(DomainAuthInfo::new(auth_password)),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainTransferRequest<F> {
        DomainTransferRequest {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type that represents the &lt;epp&gt; request for transfer approval for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::DomainTransferApprove;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an DomainTransferApprove instance
///     let domain_transfer_approve = DomainTransferApprove::<NoExtension>::new(
///         "eppdev-100.net"
///     );
///
///     // send it to the registry and receive a response of type DomainTransferApproveResponse
///     let response = client.transact(domain_transfer_approve, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainTransferApprove<E> {
    pub fn new(name: &str) -> DomainTransferApprove<NoExtension> {
        DomainTransferApprove {
            request: DomainTransferReq {
                operation: "approve".to_string(),
                domain: DomainTransferReqData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainTransferApprove<F> {
        DomainTransferApprove {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type that represents the &lt;epp&gt; request for transfer rejection for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::DomainTransferReject;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an DomainTransferReject instance
///     let domain_transfer_reject = DomainTransferReject::<NoExtension>::new(
///         "eppdev-100.net"
///     );
///
///     // send it to the registry and receive a response of type DomainTransferRejectResponse
///     let response = client.transact(domain_transfer_reject, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainTransferReject<E> {
    pub fn new(name: &str) -> DomainTransferReject<NoExtension> {
        DomainTransferReject {
            request: DomainTransferReq {
                operation: "reject".to_string(),
                domain: DomainTransferReqData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainTransferReject<F> {
        DomainTransferReject {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type that represents the &lt;epp&gt; request for transfer request cancellation for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::DomainTransferCancel;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an DomainTransferCancel instance
///     let domain_transfer_cancel = DomainTransferCancel::<NoExtension>::new(
///         "eppdev-100.net"
///     );
///
///     // send it to the registry and receive a response of type DomainTransferCancelResponse
///     let response = client.transact(domain_transfer_cancel, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainTransferCancel<E> {
    pub fn new(name: &str) -> DomainTransferCancel<NoExtension> {
        DomainTransferCancel {
            request: DomainTransferReq {
                operation: "cancel".to_string(),
                domain: DomainTransferReqData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainTransferCancel<F> {
        DomainTransferCancel {
            request: self.request,
            extension: Some(extension),
        }
    }
}

/// Type that represents the &lt;epp&gt; request for transfer request query for domains
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::transfer::DomainTransferQuery;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, RegistryConfig> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         RegistryConfig {
///             host: "example.com".to_owned(),
///             port: 700,
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
///     let login = Login::<NoExtension>::new("username", "password", None);
///     client.transact(login, "transaction-id").await.unwrap();
///
///     // Create an DomainTransferQuery instance
///     let domain_transfer_query = DomainTransferQuery::<NoExtension>::new(
///         "eppdev-100.net", "epP4uthd#v"
///     );
///
///     // send it to the registry and receive a response of type DomainTransferQueryResponse
///     let response = client.transact(domain_transfer_query, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainTransferQuery<E> {
    pub fn new(name: &str, auth_password: &str) -> DomainTransferQuery<NoExtension> {
        DomainTransferQuery {
            request: DomainTransferReq {
                operation: "query".to_string(),
                domain: DomainTransferReqData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                    period: None,
                    auth_info: Some(DomainAuthInfo::new(auth_password)),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainTransferQuery<F> {
        DomainTransferQuery {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for elements under the domain &lt;transfer&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainTransferReqData {
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
pub struct DomainTransferReq {
    /// The transfer operation to perform indicated by the 'op' attr
    /// The values are one of transfer, approve, reject, cancel, or query
    #[serde(rename = "op")]
    operation: String,
    /// The data under the &lt;transfer&gt; tag in the transfer request
    #[serde(rename = "domain:transfer")]
    domain: DomainTransferReqData,
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
