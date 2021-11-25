use epp_client_macros::*;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

use crate::epp::object::{ElementName, EppObject, Extension, StringValue};
use crate::epp::request::{CommandWithExtension, EppRequest};
use crate::epp::response::domain::check::EppDomainCheckResponse;
use crate::epp::xml::EPP_DOMAIN_XMLNS;

type EppDomainCheck<E> = EppObject<CommandWithExtension<DomainCheck, E>>;

/// Type that represents the &lt;epp&gt; request for domain &lt;check&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::object::NoExtension;
/// use epp_client::domain;
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
///     // Create an epp_client::domain::check::Request instance
///     let domain_check = domain::check::Request::<NoExtension>::new(
///         vec!["eppdev.com", "eppdev.net"],
///         None,
///         generate_client_tr_id(&client).as_str(),
///     );
///
///     // send it to the registry and receive a response of type EppDomainCheckResponse
///     let response = client.transact_new(domain_check).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct Request<E: ElementName + Serialize + DeserializeOwned + Debug>(EppDomainCheck<E>);

impl<E: ElementName + Serialize + DeserializeOwned + Debug> EppRequest for Request<E> {
    type Request = EppDomainCheck<E>;
    type Response = EppDomainCheckResponse;

    fn as_epp_object(&self) -> &Self::Request {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "check")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct DomainCheck {
    /// The object holding the list of domains to be checked
    #[serde(rename = "domain:check", alias = "check")]
    list: DomainList,
}

/// Type for &lt;name&gt; elements under the domain &lt;check&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainList {
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    /// XML namespace for domain commands
    xmlns: String,
    #[serde(rename = "domain:name", alias = "name")]
    /// List of domains to be checked for availability
    domains: Vec<StringValue>,
}

impl<E: ElementName + Serialize + DeserializeOwned + Debug> Request<E> {
    pub fn new(
        domains: impl IntoIterator<Item = impl AsRef<str>>,
        extension: Option<E>,
        client_tr_id: &str,
    ) -> Self {
        Self(EppObject::build(CommandWithExtension::<_, _> {
            command: DomainCheck {
                list: DomainList {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    domains: domains
                        .into_iter()
                        .map(|d| d.as_ref().into())
                        .collect::<Vec<StringValue>>(),
                },
            },
            extension: extension.map(|ext| Extension { data: ext }),
            client_tr_id: client_tr_id.into(),
        }))
    }
}
