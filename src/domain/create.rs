//! Types for EPP domain create request

use chrono::{DateTime, Utc};
use instant_xml::{FromXml, ToXml};

use super::{DomainAuthInfo, DomainContact, HostInfo, NameServers, Period, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for DomainCreate<'a> {}

impl<'a> Command for DomainCreate<'a> {
    type Response = DomainCreateResponse;
    const COMMAND: &'static str = "create";
}

// Request

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "create", ns(XMLNS))]
pub struct DomainCreateRequestData<'a> {
    /// The domain name
    pub name: &'a str,
    /// The period of registration
    pub period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    pub ns: Option<NameServers<'a>>,
    /// The domain registrant
    pub registrant: Option<&'a str>,
    /// The list of contacts for the domain
    pub contacts: Option<&'a [DomainContact<'a>]>,
    /// The auth info for the domain
    pub auth_info: DomainAuthInfo<'a>,
}

#[derive(Debug, ToXml)]
/// Type for EPP XML &lt;create&gt; command for domains
#[xml(rename = "create", ns(EPP_XMLNS))]
pub struct DomainCreate<'a> {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    pub domain: DomainCreateRequestData<'a>,
}

impl<'a> DomainCreate<'a> {
    pub fn new(
        name: &'a str,
        period: Period,
        ns: Option<&'a [HostInfo<'a>]>,
        registrant: Option<&'a str>,
        auth_password: &'a str,
        contacts: Option<&'a [DomainContact<'a>]>,
    ) -> Self {
        Self {
            domain: DomainCreateRequestData {
                name,
                period,
                ns: ns.map(|ns| NameServers { ns: ns.to_vec() }),
                registrant,
                auth_info: DomainAuthInfo::new(auth_password),
                contacts,
            },
        }
    }
}

// Response

/// Type that represents the &lt;chkData&gt; tag for domain create response
#[derive(Debug, FromXml)]
#[xml(rename = "creData", ns(XMLNS))]
pub struct DomainCreateResponse {
    /// The domain name
    pub name: String,
    /// The creation date
    #[xml(rename = "crDate")]
    pub created_at: DateTime<Utc>,
    /// The expiry date
    #[xml(rename = "exDate")]
    pub expiring_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use chrono::{TimeZone, Utc};

    use super::{DomainContact, DomainCreate, Period};
    use crate::domain::{HostAttr, HostInfo, HostObj};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let contacts = &[
            DomainContact {
                contact_type: "admin".into(),
                id: "eppdev-contact-3".into(),
            },
            DomainContact {
                contact_type: "tech".into(),
                id: "eppdev-contact-3".into(),
            },
            DomainContact {
                contact_type: "billing".into(),
                id: "eppdev-contact-3".into(),
            },
        ];

        let object = DomainCreate::new(
            "eppdev-1.com",
            Period::years(1).unwrap(),
            None,
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        assert_serialized("request/domain/create.xml", &object);
    }

    #[test]
    fn command_with_host_obj() {
        let contacts = &[
            DomainContact {
                contact_type: "admin".into(),
                id: "eppdev-contact-3".into(),
            },
            DomainContact {
                contact_type: "tech".into(),
                id: "eppdev-contact-3".into(),
            },
            DomainContact {
                contact_type: "billing".into(),
                id: "eppdev-contact-3".into(),
            },
        ];

        let hosts = &[
            HostInfo::Obj(HostObj {
                name: "ns1.test.com".into(),
            }),
            HostInfo::Obj(HostObj {
                name: "ns2.test.com".into(),
            }),
        ];
        let object = DomainCreate::new(
            "eppdev-1.com",
            Period::years(1).unwrap(),
            Some(hosts),
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        assert_serialized("request/domain/create_with_host_obj.xml", &object);
    }

    #[test]
    fn command_with_host_attr() {
        let contacts = &[
            DomainContact {
                contact_type: "admin".into(),
                id: "eppdev-contact-3".into(),
            },
            DomainContact {
                contact_type: "tech".into(),
                id: "eppdev-contact-3".into(),
            },
            DomainContact {
                contact_type: "billing".into(),
                id: "eppdev-contact-3".into(),
            },
        ];

        let hosts = &[
            HostInfo::Attr(HostAttr {
                name: "ns1.eppdev-1.com".into(),
                addresses: None,
            }),
            HostInfo::Attr(HostAttr {
                name: "ns2.eppdev-1.com".into(),
                addresses: Some(vec![
                    IpAddr::from([177, 232, 12, 58]),
                    IpAddr::from([0x2404, 0x6800, 0x4001, 0x801, 0, 0, 0, 0x200e]),
                ]),
            }),
        ];

        let object = DomainCreate::new(
            "eppdev-2.com",
            Period::years(1).unwrap(),
            Some(hosts),
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        assert_serialized("request/domain/create_with_host_attr.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<DomainCreate>("response/domain/create.xml");

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.name, "eppdev-2.com");
        assert_eq!(
            result.created_at,
            Utc.with_ymd_and_hms(2021, 7, 25, 18, 11, 35).unwrap()
        );
        assert_eq!(
            *result.expiring_at.as_ref().unwrap(),
            Utc.with_ymd_and_hms(2022, 7, 25, 18, 11, 34).unwrap()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
