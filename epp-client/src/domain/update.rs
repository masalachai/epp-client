//! Types for EPP domain check request

use epp_client_macros::*;

use crate::{
    common::{
        DomainAuthInfo, DomainContact, DomainStatus, ElementName, HostList, NoExtension,
        StringValue,
    },
    request::{EppExtension, EppRequest},
};

use super::EPP_DOMAIN_XMLNS;

use crate::response::EppCommandResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainUpdate<E> {
    request: DomainUpdateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainUpdate<E> {
    type Input = DomainUpdateRequest;
    type Output = EppCommandResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for domain &lt;update&gt; command
/// with &lt;hostObj&gt; elements in the request for &lt;ns&gt; list
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::common::{DomainStatus, DomainContact};
/// use epp_client::domain::update::{DomainUpdate, DomainAddRemove};
/// use epp_client::generate_client_tr_id;
/// use epp_client::common::NoExtension;
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
///     // Create an DomainUpdate instance
///     let mut domain_update = DomainUpdate::<NoExtension>::new("eppdev-100.com");
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
///     // send it to the registry and receive a response of type EppDomainUpdateResponse
///     let response = client.transact(domain_update, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainUpdate<E> {
    pub fn new(name: &str) -> DomainUpdate<NoExtension> {
        DomainUpdate {
            request: DomainUpdateRequest {
                domain: DomainUpdateRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    add: None,
                    remove: None,
                    change_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainUpdate<F> {
        DomainUpdate {
            request: self.request,
            extension: Some(extension),
        }
    }

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.request.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove) {
        self.request.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove) {
        self.request.domain.remove = Some(remove);
    }
}

/// Type for elements under the &lt;chg&gt; tag for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainChangeInfo {
    /// The new registrant contact for the domain
    #[serde(rename = "domain:registrant", alias = "update")]
    pub registrant: Option<StringValue>,
    /// The new auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    pub auth_info: Option<DomainAuthInfo>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAddRemove {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    pub ns: Option<HostList>,
    /// The list of contacts to add to or remove from the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of statuses to add to or remove from the domain
    #[serde(rename = "domain:status", alias = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
}

/// Type for elements under the &lt;update&gt; tag for domain update
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainUpdateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    pub xmlns: String,
    /// The name of the domain to update
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    #[serde(rename = "domain:add", alias = "add")]
    pub add: Option<DomainAddRemove>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    #[serde(rename = "domain:rem", alias = "rem")]
    pub remove: Option<DomainAddRemove>,
    /// The data under the &lt;chg&gt; tag for domain update
    #[serde(rename = "domain:chg", alias = "chg")]
    pub change_info: Option<DomainChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for domains
pub struct DomainUpdateRequest {
    #[serde(rename = "domain:update", alias = "update")]
    pub domain: DomainUpdateRequestData,
}
