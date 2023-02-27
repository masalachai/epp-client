//! Types for EPP domain renew request

use chrono::{DateTime, NaiveDate, Utc};
use instant_xml::{FromXml, ToXml};

use super::{Period, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for DomainRenew<'a> {}

impl<'a> Command for DomainRenew<'a> {
    type Response = RenewData;
    const COMMAND: &'static str = "renew";
}

impl<'a> DomainRenew<'a> {
    pub fn new(name: &'a str, current_expiry_date: NaiveDate, period: Period) -> Self {
        Self {
            domain: DomainRenewRequestData {
                name,
                current_expiry_date,
                period,
            },
        }
    }
}

// Request

/// Type for data under the domain &lt;renew&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "renew", ns(XMLNS))]
pub struct DomainRenewRequestData<'a> {
    /// The name of the domain to be renewed
    name: &'a str,
    /// The current expiry date of the domain in 'Y-m-d' format
    #[xml(rename = "curExpDate")]
    current_expiry_date: NaiveDate,
    /// The period of renewal
    period: Period,
}

#[derive(Debug, ToXml)]
/// Type for EPP XML &lt;renew&gt; command for domains
#[xml(rename = "renew", ns(EPP_XMLNS))]
pub struct DomainRenew<'a> {
    /// The data under the &lt;renew&gt; tag for the domain renewal
    #[xml(rename = "renew")]
    domain: DomainRenewRequestData<'a>,
}

// Response

/// Type that represents the &lt;renData&gt; tag for domain renew response
#[derive(Debug, FromXml)]
#[xml(rename = "renData", ns(XMLNS))]
pub struct RenewData {
    /// The name of the domain
    pub name: String,
    /// The new expiry date after renewal
    #[xml(rename = "exDate")]
    pub expiring_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::{DomainRenew, Period};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    use chrono::{NaiveDate, TimeZone, Utc};

    #[test]
    fn command() {
        let exp_date = NaiveDate::from_ymd_opt(2022, 7, 23).unwrap();
        let object = DomainRenew::new("eppdev.com", exp_date, Period::years(1).unwrap());
        assert_serialized("request/domain/renew.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<DomainRenew>("response/domain/renew.xml");

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.name, "eppdev-1.com");
        assert_eq!(
            *result.expiring_at.as_ref().unwrap(),
            Utc.with_ymd_and_hms(2024, 7, 23, 15, 31, 20).unwrap()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
