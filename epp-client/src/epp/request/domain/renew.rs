//! Types for EPP domain renew request

use epp_client_macros::*;

use crate::epp::object::data::Period;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for domain <renew> command
pub type EppDomainRenew = EppObject<Command<DomainRenew>>;

/// Type for data under the domain <renew> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewData {
    /// XML namespace for domain commands
    xmlns: String,
    /// The name of the domain to be renewed
    name: StringValue,
    /// The current expiry date of the domain in 'Y-m-d' format
    #[serde(rename = "curExpDate")]
    current_expiry_date: StringValue,
    /// The period of renewal
    period: Period,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "renew")]
/// Type for EPP XML <renew> command for domains
pub struct DomainRenew {
    /// The data under the <renew> tag for the domain renewal
    #[serde(rename = "renew")]
    domain: DomainRenewData,
}

impl EppDomainRenew {
    /// Creates a new EppObject for domain renew corresponding to the <epp> tag in EPP XML
    pub fn new(
        name: &str,
        current_expiry_date: NaiveDate,
        years: u16,
        client_tr_id: &str,
    ) -> EppDomainRenew {
        let exp_date_str = current_expiry_date
            .format("%Y-%m-%d")
            .to_string()
            .to_string_value();

        EppObject::build(Command::<DomainRenew> {
            command: DomainRenew {
                domain: DomainRenewData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    current_expiry_date: exp_date_str,
                    period: Period::new(years),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    pub fn set_period(&mut self, period: Period) {
        self.data.command.domain.period = period;
    }
}
