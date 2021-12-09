//! Types for EPP domain renew request

use super::XMLNS;
use crate::common::{NoExtension, Period, StringValue};
use crate::request::{Command, Transaction};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainRenew {}

impl Command for DomainRenew {
    type Response = DomainRenewResponse;
    const COMMAND: &'static str = "renew";
}

impl DomainRenew {
    pub fn new(name: &str, current_expiry_date: NaiveDate, years: u16) -> Self {
        let exp_date_str = current_expiry_date.format("%Y-%m-%d").to_string().into();
        Self {
            domain: DomainRenewRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                current_expiry_date: exp_date_str,
                period: Period::new(years),
            },
        }
    }
}

// Request

/// Type for data under the domain &lt;renew&gt; tag
#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;renew&gt; command for domains
pub struct DomainRenew {
    /// The data under the &lt;renew&gt; tag for the domain renewal
    #[serde(rename = "domain:renew", alias = "renew")]
    domain: DomainRenewRequestData,
}

// Response

/// Type that represents the &lt;renData&gt; tag for domain renew response
#[derive(Deserialize, Debug)]
pub struct DomainRenewResponseData {
    /// The name of the domain
    pub name: StringValue,
    /// The new expiry date after renewal
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain renew response
#[derive(Deserialize, Debug)]
pub struct DomainRenewResponse {
    /// Data under the &lt;renData&gt; tag
    #[serde(rename = "renData")]
    pub renew_data: DomainRenewResponseData,
}
