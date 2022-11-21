//! Types for EPP consolidate request

use std::fmt;

use chrono::FixedOffset;
use serde::Serialize;

use crate::common::{NoExtension, StringValue};
use crate::domain::update::DomainUpdate;
use crate::request::{Extension, Transaction};

use super::namestore::{NameStore, NameStoreData};

pub const XMLNS: &str = "http://www.verisign.com/epp/sync-1.0";

impl Transaction<Update> for DomainUpdate<'_> {}

impl Extension for Update {
    type Response = NoExtension;
}

impl Transaction<UpdateWithNameStore<'_>> for DomainUpdate<'_> {}

impl Extension for UpdateWithNameStore<'_> {
    type Response = NameStore<'static>;
}

#[derive(PartialEq, Eq, Debug)]
pub struct GMonthDay {
    pub month: u8,
    pub day: u8,
    pub timezone: Option<FixedOffset>,
}

// Taken from https://github.com/lumeohq/xsd-parser-rs/blob/main/xsd-types/src/types/gmonthday.rs
/// Represents a gMonthDay type <https://www.w3.org/TR/xmlschema-2/#gMonthDay>
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
    /// Create a new sync update request
    pub fn new(expiration: GMonthDay) -> Self {
        Self {
            data: UpdateData {
                xmlns: XMLNS,
                exp: expiration.to_string().into(),
            },
        }
    }
}

impl UpdateWithNameStore<'_> {
    /// Create a new sync update with namestore request
    pub fn new(expiration: GMonthDay, subproduct: &str) -> Self {
        Self {
            sync: Update::new(expiration).data,
            namestore: NameStoreData::new(subproduct),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Update {
    #[serde(rename = "sync:update")]
    pub data: UpdateData,
}

#[derive(Debug, Serialize)]
pub struct UpdateWithNameStore<'a> {
    #[serde(rename = "sync:update")]
    pub sync: UpdateData,
    #[serde(rename = "namestoreExt:namestoreExt")]
    pub namestore: NameStoreData<'a>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;consolidate&gt; extension
pub struct UpdateData {
    /// XML namespace for the consolidate extension
    #[serde(rename = "xmlns:sync")]
    pub xmlns: &'static str,
    /// The expiry date of the domain
    #[serde(rename = "sync:expMonthDay")]
    pub exp: StringValue<'static>,
}

#[cfg(test)]
mod tests {
    use super::{GMonthDay, Update};
    use crate::domain::update::{DomainChangeInfo, DomainUpdate};
    use crate::extensions::consolidate::UpdateWithNameStore;
    use crate::tests::assert_serialized;

    #[test]
    fn command() {
        let exp = GMonthDay::new(5, 31, None).unwrap();

        let consolidate_ext = Update::new(exp);

        let mut object = DomainUpdate::new("eppdev.com");

        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        assert_serialized(
            "request/extensions/consolidate.xml",
            (&object, &consolidate_ext),
        );
    }

    #[test]
    fn command_with_namestore() {
        let exp = GMonthDay::new(5, 31, None).unwrap();

        let consolidate_ext = UpdateWithNameStore::new(exp, "com");

        let mut object = DomainUpdate::new("eppdev.com");

        object.info(DomainChangeInfo {
            registrant: None,
            auth_info: None,
        });

        assert_serialized(
            "request/extensions/consolidate_namestore.xml",
            (&object, &consolidate_ext),
        );
    }
}
