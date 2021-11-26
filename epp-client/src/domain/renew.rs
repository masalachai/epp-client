//! Types for EPP domain renew request

use epp_client_macros::*;

use crate::common::{ElementName, EppObject, Period, StringValue};
use crate::epp::request::Command;
use crate::epp::response::CommandResponse;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;renew&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use chrono::NaiveDate;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::domain::renew::{EppDomainRenew, EppDomainRenewResponse};
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
///     // Create a date object to set the current expiry date
///     let exp_date = NaiveDate::from_ymd(2022, 7, 27);
///
///     // Create an EppDomainRenew instance
///     let domain_renew = EppDomainRenew::new("eppdev-100.com", exp_date, 1, generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppDomainRenewResponse
///     let response = client.transact::<_, EppDomainRenewResponse>(&domain_renew).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainRenew = EppObject<Command<DomainRenewRequest>>;

impl EppDomainRenew {
    /// Creates a new EppObject for domain renew corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(
        name: &str,
        current_expiry_date: NaiveDate,
        years: u16,
        client_tr_id: &str,
    ) -> EppDomainRenew {
        let exp_date_str = current_expiry_date.format("%Y-%m-%d").to_string().into();

        EppObject::build(Command::<DomainRenewRequest>::new(
            DomainRenewRequest {
                domain: DomainRenewRequestData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    current_expiry_date: exp_date_str,
                    period: Period::new(years),
                },
            },
            client_tr_id,
        ))
    }

    pub fn set_period(&mut self, period: Period) {
        self.data.command.domain.period = period;
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain renew response
pub type EppDomainRenewResponse = EppObject<CommandResponse<DomainRenewResponse>>;

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
