//! Types for EPP RGP restore request

use epp_client_macros::*;

use crate::common::ElementName;
use crate::domain::rgp::EPP_DOMAIN_RGP_EXT_XMLNS;

use crate::request::EppExtension;

use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for a domain rgp restore request command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::rgp::request::RgpRestoreRequest;
/// use epp_client::domain::update::DomainUpdate;
/// use epp_client::generate_client_tr_id;
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
///     // Create an RgpRestoreRequest instance
///     let domain_restore_req = RgpRestoreRequest::new();
///
///     // Create an DomainUpdate instance
///     let mut domain_update = DomainUpdate::<RgpRestoreRequest>::new("eppdev-100.com").with_extension(domain_restore_req);
///
///     // send it to the registry and receive a response of type EppDomainUpdateResponse
///     let response = client.transact(domain_update, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl RgpRestoreRequest {
    /// Creates a new instance of EppDomainRgpRestoreRequest
    pub fn new() -> RgpRestoreRequest {
        RgpRestoreRequest {
            xmlns: EPP_DOMAIN_RGP_EXT_XMLNS.to_string(),
            restore: RgpRestoreRequestData {
                op: "request".to_string(),
            },
        }
    }
}

impl Default for RgpRestoreRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl EppExtension for RgpRestoreRequest {
    type Response = RgpRequestResponse;
}

// Request

/// Type corresponding to the &lt;restore&gt; tag for an rgp restore request
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpRestoreRequestData {
    /// The value of the op attribute in the &lt;restore&gt; tag
    pub op: String,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "rgp:update")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreRequest {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreRequestData,
}

// Response

/// Type that represents the &lt;rgpStatus&gt; tag for domain rgp restore request response
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpStatus {
    /// The domain RGP status
    #[serde(rename = "s")]
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[serde(rename = "upData")]
#[element_name(name = "upData")]
/// Type that represents the &lt;resData&gt; tag for domain transfer response
pub struct RgpRequestResponse {
    #[serde(rename = "xmlns:rgp")]
    xmlns: String,
    /// Data under the &lt;rgpStatus&gt; tag
    #[serde(rename = "rgpStatus")]
    pub rgp_status: RgpStatus,
}
