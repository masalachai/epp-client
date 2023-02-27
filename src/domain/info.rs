//! Types for EPP domain info request

use chrono::{DateTime, Utc};
use instant_xml::{FromXml, ToXml};

use super::{DomainAuthInfo, DomainContact, HostAttr, NameServers, Status, XMLNS};
use crate::common::{NoExtension, EPP_XMLNS};
use crate::request::{Command, Transaction};

impl<'a> Transaction<NoExtension> for DomainInfo<'a> {}

impl<'a> Command for DomainInfo<'a> {
    type Response = InfoData;
    const COMMAND: &'static str = "info";
}

impl<'a> DomainInfo<'a> {
    pub fn new(name: &'a str, auth_password: Option<&'a str>) -> Self {
        Self {
            info: DomainInfoRequestData {
                name: Domain { hosts: "all", name },
                auth_info: auth_password.map(|password| DomainAuthInfo {
                    password: password.into(),
                }),
            },
        }
    }
}

// Request

/// Type for data under the &lt;name&gt; element tag for the domain &lt;info&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "name", ns(XMLNS))]
pub struct Domain<'a> {
    /// The hosts attribute. Default value is "all"
    #[xml(attribute)]
    hosts: &'a str,
    /// The name of the domain
    #[xml(direct)]
    name: &'a str,
}

/// Type for &lt;name&gt; element under the domain &lt;info&gt; tag
#[derive(Debug, ToXml)]
#[xml(rename = "info", ns(XMLNS))]
pub struct DomainInfoRequestData<'a> {
    /// The data for the domain to be queried
    name: Domain<'a>,
    /// The auth info for the domain
    auth_info: Option<DomainAuthInfo<'a>>,
}

#[derive(Debug, ToXml)]
/// Type for EPP XML &lt;info&gt; command for domains
#[xml(rename = "info", ns(EPP_XMLNS))]
pub struct DomainInfo<'a> {
    /// The data under the &lt;info&gt; tag for domain info
    info: DomainInfoRequestData<'a>,
}

// Response

/// The two types of ns lists, hostObj and hostAttr, that may be returned in the
/// domain info response
#[derive(Debug, FromXml)]
pub struct DomainNsList {
    /// List of &lt;hostObj&gt; ns elements
    #[xml(rename = "hostObj")]
    pub host_obj: Option<Vec<String>>,
    /// List of &lt;hostAttr&gt; ns elements
    pub host_attr: Option<Vec<HostAttr<'static>>>,
}

/// Type that represents the &lt;infData&gt; tag for domain info response
#[derive(Debug, FromXml)]
#[xml(rename = "infData", ns(XMLNS))]
pub struct InfoData {
    /// The domain name
    pub name: String,
    /// The domain ROID
    pub roid: String,
    /// The list of domain statuses
    #[xml(rename = "status")]
    pub statuses: Option<Vec<Status<'static>>>,
    /// The domain registrant
    pub registrant: Option<String>,
    /// The list of domain contacts
    #[xml(rename = "contact")]
    pub contacts: Option<Vec<DomainContact<'static>>>,
    /// The list of domain nameservers
    pub ns: Option<NameServers<'static>>,
    /// The list of domain hosts
    #[xml(rename = "host")]
    pub hosts: Option<Vec<String>>,
    /// The epp user who owns the domain
    #[xml(rename = "clID")]
    pub client_id: String,
    /// The epp user who created the domain
    #[xml(rename = "crID")]
    pub creator_id: Option<String>,
    /// The domain creation date
    #[xml(rename = "crDate")]
    pub created_at: Option<DateTime<Utc>>,
    /// The domain expiry date
    #[xml(rename = "exDate")]
    pub expiring_at: Option<DateTime<Utc>>,
    /// The epp user who last updated the domain
    #[xml(rename = "upID")]
    pub updater_id: Option<String>,
    /// The domain last updated date
    #[xml(rename = "upDate")]
    pub updated_at: Option<DateTime<Utc>>,
    /// The domain transfer date
    #[xml(rename = "trDate")]
    pub transferred_at: Option<DateTime<Utc>>,
    /// The domain auth info
    #[xml(rename = "authInfo")]
    pub auth_info: Option<DomainAuthInfo<'static>>,
}

#[cfg(test)]
mod tests {
    use super::DomainInfo;
    use crate::domain::{HostInfo, HostObj};
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};
    use chrono::{TimeZone, Utc};

    #[test]
    fn command() {
        let object = DomainInfo::new("eppdev.com", Some("2fooBAR"));
        assert_serialized("request/domain/info.xml", &object);
    }

    #[test]
    fn response() {
        let object = response_from_file::<DomainInfo>("response/domain/info.xml");
        dbg!(&object);

        let result = object.res_data().unwrap();
        let auth_info = result.auth_info.as_ref().unwrap();
        let ns = result.ns.as_ref().unwrap();
        let hosts = result.hosts.as_ref().unwrap();
        let statuses = result.statuses.as_ref().unwrap();
        let registrant = result.registrant.as_ref().unwrap();
        let contacts = result.contacts.as_ref().unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(result.name, "eppdev-1.com");
        assert_eq!(result.roid, "125899511_DOMAIN_COM-VRSN");
        assert_eq!(statuses[0].status, "ok".to_string());
        assert_eq!(statuses[1].status, "clientTransferProhibited".to_string());
        assert_eq!(*registrant, "eppdev-contact-2");
        assert_eq!(contacts[0].id, "eppdev-contact-2".to_string());
        assert_eq!(contacts[0].contact_type, "admin".to_string());
        assert_eq!(contacts[1].id, "eppdev-contact-2".to_string());
        assert_eq!(contacts[1].contact_type, "tech".to_string());
        assert_eq!(contacts[2].id, "eppdev-contact-2".to_string());
        assert_eq!(contacts[2].contact_type, "billing".to_string());
        assert_eq!(
            ns.ns[0],
            HostInfo::Obj(HostObj {
                name: "ns1.eppdev-1.com".into()
            })
        );
        assert_eq!(
            ns.ns[1],
            HostInfo::Obj(HostObj {
                name: "ns2.eppdev-1.com".into()
            })
        );
        assert_eq!((*hosts)[0], "ns1.eppdev-1.com");
        assert_eq!((*hosts)[1], "ns2.eppdev-1.com");
        assert_eq!(result.client_id, "eppdev");
        assert_eq!(*result.creator_id.as_ref().unwrap(), "SYSTEM");
        assert_eq!(
            *result.created_at.as_ref().unwrap(),
            Utc.with_ymd_and_hms(2021, 7, 23, 15, 31, 20).unwrap()
        );
        assert_eq!(*result.updater_id.as_ref().unwrap(), "SYSTEM");
        assert_eq!(
            *result.updated_at.as_ref().unwrap(),
            Utc.with_ymd_and_hms(2021, 7, 23, 15, 31, 21).unwrap()
        );
        assert_eq!(
            *result.expiring_at.as_ref().unwrap(),
            Utc.with_ymd_and_hms(2023, 7, 23, 15, 31, 20).unwrap()
        );
        assert_eq!(auth_info.password, "epP4uthd#v");
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }

    #[test]
    fn response_alt() {
        response_from_file::<DomainInfo>("response/domain/info_alt.xml");
    }
}
