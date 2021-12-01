//! Types for EPP consolidate request

use std::fmt;

use chrono::FixedOffset;
use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension, StringValue},
    request::EppExtension,
};

pub const XMLNS: &str = "http://www.verisign.com/epp/sync-1.0";

#[derive(PartialEq, Debug)]
pub struct GMonthDay {
    pub month: u8,
    pub day: u8,
    pub timezone: Option<FixedOffset>,
}

// Taken from https://github.com/lumeohq/xsd-parser-rs/blob/main/xsd-types/src/types/gmonthday.rs
/// Represents a gMonthDay type https://www.w3.org/TR/xmlschema-2/#gMonthDay
impl GMonthDay {
    pub fn new(month: u8, day: u8, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if !(1..=12).contains(&month) {
            return Err("Month value within GMonthDay should lie between 1 and 12".to_string());
        }

        if !(1..=31).contains(&day) {
            return Err("Day value within GMonthDay should lie between 1 and 31".to_string());
        }

        const MONTH_MAX_LEN: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if day > MONTH_MAX_LEN[month as usize - 1] {
            return Err("Day value within GMonthDay is to big for specified month".to_string());
        }

        Ok(GMonthDay {
            month,
            day,
            timezone,
        })
    }
}

impl fmt::Display for GMonthDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "--{:02}-{:02}{}", self.month, self.day, tz),
            None => write!(f, "--{:02}-{:02}", self.month, self.day),
        }
    }
}

/// Type that represents the domain rgp restore report extension
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::common::{DomainStatus, DomainContact};
/// use epp_client::extensions::rgp::report::RgpRestoreReport;
/// use epp_client::domain::update::DomainUpdate;
/// use epp_client::generate_client_tr_id;
/// use epp_client::common::NoExtension;
/// use chrono::{DateTime, NaiveDate};
/// use std::str::FromStr;
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
///     let domain_restore_report = RgpRestoreReport::new(
///         pre_data,
///         post_data,
///         deleted_at,
///         restored_at,
///         restore_reason,
///         &statements,
///         other
///     );
///
///     // Create an DomainUpdate instance
///     let mut domain_update = DomainUpdate::<RgpRestoreReport>::new("eppdev-100.com").with_extension(domain_restore_report);
///
///     // send it to the registry and receive a response of type EppDomainUpdateResponse
///     let response = client.transact(domain_update, generate_client_tr_id(&client).as_str()).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
impl Sync {
    /// Create a new RGP restore report request
    pub fn new(expiration: GMonthDay) -> Sync {
        Sync {
            xmlns: XMLNS.to_string(),
            exp: expiration.to_string().into(),
        }
    }
}

impl EppExtension for Sync {
    type Response = NoExtension;
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "sync:update")]
/// Type for EPP XML &lt;consolidate&gt; extension
pub struct Sync {
    /// XML namespace for the consolidate extension
    #[serde(rename = "xmlns:sync", alias = "xmlns")]
    pub xmlns: String,
    /// The expiry date of the domain
    #[serde(rename = "sync:expMonthDay", alias = "sync")]
    pub exp: StringValue,
}
