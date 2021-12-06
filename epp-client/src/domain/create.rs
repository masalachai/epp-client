//! Types for EPP domain create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{
    DomainAuthInfo, DomainContact, ElementName, HostList, NoExtension, Period, StringValue,
};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainCreate<E> {
    request: DomainCreateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for DomainCreate<E> {
    type Input = DomainCreateRequest;
    type Output = DomainCreateResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for domain &lt;create&gt; command
/// with &lt;hostObj&gt; elements in the request for &lt;ns&gt; list
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::common::DomainContact;
/// use epp_client::domain::create::DomainCreate;
/// use epp_client::common::NoExtension;
/// use epp_client::login::Login;
/// use epp_client::logout::Logout;
/// use epp_client::common::HostAttrList;
/// use epp_client::common::HostList;
/// use epp_client::common::HostObjList;

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
///     let contacts = vec![
///         DomainContact {
///             contact_type: "admin".to_string(),
///             id: "eppdev-contact-3".to_string(),
///         },
///         DomainContact {
///             contact_type: "tech".to_string(),
///             id: "eppdev-contact-3".to_string(),
///         },
///         DomainContact {
///             contact_type: "billing".to_string(),
///             id: "eppdev-contact-3".to_string(),
///         },
///     ];

///     let ns = Some(HostList::HostObjList(HostObjList {
///         hosts: vec!["ns1.test.com".into(), "ns2.test.com".into()],
///     }));

///     let domain_create = DomainCreate::<NoExtension>::new(
///         "eppdev-1.com",
///         1,
///         ns,
///         Some("eppdev-contact-3"),
///         "epP4uthd#v",
///         Some(contacts),
///     );
///
///     // send it to the registry and receive a response of type EppDomainCreateResponse
///     let response = client.transact(domain_create, "transaction-id").await.unwrap();
///
///     println!("{:?}", response);
///
///     let logout = Logout::<NoExtension>::new();
///     client.transact(logout, "transaction-id").await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainCreate<E> {
    pub fn new(
        name: &str,
        period: u16,
        ns: Option<HostList>,
        registrant_id: Option<&str>,
        auth_password: &str,
        contacts: Option<Vec<DomainContact>>,
    ) -> DomainCreate<NoExtension> {
        let registrant = registrant_id.map(|id| id.into());
        let domain_create = DomainCreateRequest {
            domain: DomainCreateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns,
                registrant,
                auth_info: DomainAuthInfo::new(auth_password),
                contacts,
            },
        };

        DomainCreate {
            request: domain_create,
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainCreate<F> {
        DomainCreate {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    pub xmlns: String,
    /// The domain name
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// The period of registration
    #[serde(rename = "domain:period", alias = "period")]
    pub period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    pub ns: Option<HostList>,
    /// The domain registrant
    #[serde(rename = "domain:registrant", alias = "registrant")]
    pub registrant: Option<StringValue>,
    /// The list of contacts for the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    pub auth_info: DomainAuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for domains
pub struct DomainCreateRequest {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "domain:create", alias = "create")]
    pub domain: DomainCreateRequestData,
}

// Response

/// Type that represents the &lt;chkData&gt; tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    pub xmlns: String,
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
