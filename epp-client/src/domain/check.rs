use epp_client_macros::*;

use serde::{Deserialize, Serialize};

use crate::epp::object::{ElementName, EmptyTag, StringValue};
use crate::epp::request::EppRequest;
use crate::epp::xml::EPP_DOMAIN_XMLNS;

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::check::Check;
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
///     // Create a Check instance
///     let domain_check = Check::new(vec!["eppdev-100.com", "eppdev-100.net"];
///
///     // send it to the registry and receive a response of type DomainCheck
///     let response = client.transact_new::<_, DomainCheck>(&domain_check).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

#[derive(Debug)]
pub struct DomainCheck {
    request: DomainCheckRequest,
}

impl EppRequest for DomainCheck {
    type Input = DomainCheckRequest;
    type InputExtension = EmptyTag;
    type Output = DomainCheckResponse;
    type OutputExtension = EmptyTag;

    fn into_parts(self) -> (Self::Input, Option<Self::InputExtension>) {
        (self.request, None)
    }
}

impl DomainCheck {
    pub fn new(domains: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        Self {
            request: DomainCheckRequest {
                list: DomainCheckList {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domains: domains
                        .into_iter()
                        .map(|d| d.as_ref().into())
                        .collect::<Vec<StringValue>>(),
                },
            },
        }
    }
}

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckList {
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    /// XML namespace for domain commands
    pub xmlns: String,
    #[serde(rename = "domain:name", alias = "name")]
    /// List of domains to be checked for availability
    pub domains: Vec<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheckRequest {
    /// The object holding the list of domains to be checked
    #[serde(rename = "domain:check", alias = "check")]
    list: DomainCheckList,
}

/// Type that represents the &lt;name&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainAvailableData {
    /// The domain name
    #[serde(rename = "$value")]
    pub name: StringValue,
    /// The domain (un)availability
    #[serde(rename = "avail")]
    pub available: u16,
}

/// Type that represents the &lt;cd&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponseDataItem {
    /// Data under the &lt;name&gt; tag
    #[serde(rename = "name")]
    pub domain: DomainAvailableData,
    /// The reason for (un)availability
    pub reason: Option<StringValue>,
}

/// Type that represents the &lt;chkData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// Data under the &lt;cd&gt; tag
    #[serde(rename = "cd")]
    pub domain_list: Vec<DomainCheckResponseDataItem>,
}

/// Type that represents the &lt;resData&gt; tag for domain check response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCheckResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "chkData")]
    pub check_data: DomainCheckResponseData,
}
