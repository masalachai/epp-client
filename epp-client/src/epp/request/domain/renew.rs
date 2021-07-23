use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::Period;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type EppDomainRenew = EppObject<Command<DomainRenew>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewData {
    xmlns: String,
    name: StringValue,
    #[serde(rename = "curExpDate")]
    current_expiry_date: StringValue,
    period: Period,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "renew")]
pub struct DomainRenew {
    #[serde(rename = "renew")]
    domain: DomainRenewData,
}

impl EppDomainRenew {
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
