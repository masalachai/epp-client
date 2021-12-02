//! Types for EPP domain renew request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, NoExtension, Period, StringValue};
use crate::request::{EppExtension, EppRequest};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainRenew<E> {
    request: DomainRenewRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for DomainRenew<E> {
    type Input = DomainRenewRequest;
    type Output = DomainRenewResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

/// Type that represents the &lt;epp&gt; request for domain &lt;renew&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use chrono::NaiveDate;
///
/// use epp_client::config::{EppClientConfig, RegistryConfig};
/// use epp_client::EppClient;
/// use epp_client::domain::renew::DomainRenew;
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
///     // Create a date object to set the current expiry date
///     let exp_date = NaiveDate::from_ymd(2022, 7, 27);
///
///     // Create an DomainRenew instance
///     let domain_renew = DomainRenew::<NoExtension>::new("eppdev-100.com", exp_date, 1);
///
///     // send it to the registry and receive a response of type DomainRenewResponse
///     let response = client.transact(domain_renew, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl<E: EppExtension> DomainRenew<E> {
    pub fn new(name: &str, current_expiry_date: NaiveDate, years: u16) -> DomainRenew<NoExtension> {
        let exp_date_str = current_expiry_date.format("%Y-%m-%d").to_string().into();
        DomainRenew {
            request: DomainRenewRequest {
                domain: DomainRenewRequestData {
                    xmlns: XMLNS.to_string(),
                    name: name.into(),
                    current_expiry_date: exp_date_str,
                    period: Period::new(years),
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainRenew<F> {
        DomainRenew {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for data under the domain &lt;renew&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The name of the domain to be renewed
    #[serde(rename = "domain:name", alias = "name")]
    name: StringValue,
    /// The current expiry date of the domain in 'Y-m-d' format
    #[serde(rename = "domain:curExpDate", alias = "curExpDate")]
    current_expiry_date: StringValue,
    /// The period of renewal
    #[serde(rename = "domain:period", alias = "period")]
    period: Period,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "renew")]
/// Type for EPP XML &lt;renew&gt; command for domains
pub struct DomainRenewRequest {
    /// The data under the &lt;renew&gt; tag for the domain renewal
    #[serde(rename = "domain:renew", alias = "renew")]
    domain: DomainRenewRequestData,
}

// Response

/// Type that represents the &lt;renData&gt; tag for domain renew response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The name of the domain
    pub name: StringValue,
    /// The new expiry date after renewal
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain renew response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewResponse {
    /// Data under the &lt;renData&gt; tag
    #[serde(rename = "renData")]
    pub renew_data: DomainRenewResponseData,
}
