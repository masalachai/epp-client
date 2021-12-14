//! Types for EPP consolidate request

use std::fmt;

use chrono::FixedOffset;
use serde::Serialize;

use crate::common::{NoExtension, StringValue};
use crate::domain::update::DomainUpdate;
use crate::request::{Extension, Transaction};

pub const XMLNS: &str = "http://www.verisign.com/epp/sync-1.0";

impl Transaction<Update> for DomainUpdate {}

impl Extension for Update {
    type Response = NoExtension;
}

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

impl Update {
    /// Create a new RGP restore report request
    pub fn new(expiration: GMonthDay) -> Self {
        Self {
            data: UpdateData {
                xmlns: XMLNS.to_string(),
                exp: expiration.to_string().into(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Update {
    #[serde(rename = "sync:update")]
    pub data: UpdateData,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;consolidate&gt; extension
pub struct UpdateData {
    /// XML namespace for the consolidate extension
    #[serde(rename = "xmlns:sync")]
    pub xmlns: String,
    /// The expiry date of the domain
    #[serde(rename = "sync:expMonthDay")]
    pub exp: StringValue,
}

#[cfg(test)]
mod tests {
    use super::{GMonthDay, Update};
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/extensions/consolidate.xml").unwrap();

        let exp = GMonthDay::new(5, 31, None).unwrap();

        let consolidate_ext = Update::new(exp);

        let mut object = DomainUpdate::new("eppdev.com");

        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        let serialized = <DomainUpdate as Transaction<Update>>::serialize_request(
            &object,
            Some(&consolidate_ext),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }
}
