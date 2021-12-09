//! Types for EPP domain info request

use super::{DomainAuthInfo, DomainContact, HostAttr, XMLNS};
use crate::common::{DomainStatus, NoExtension, StringValue};
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainInfo {}

impl Command for DomainInfo {
    type Response = DomainInfoResponse;
    const COMMAND: &'static str = "info";
}

impl DomainInfo {
    pub fn new(name: &str, auth_password: Option<&str>) -> Self {
        Self {
            info: DomainInfoRequestData {
                xmlns: XMLNS.to_string(),
                domain: Domain {
                    hosts: "all".to_string(),
                    name: name.to_string(),
                },
                auth_info: auth_password.map(|password| DomainAuthInfo {
                    password: password.into(),
                }),
            },
        }
    }
}

// Request

/// Type for data under the &lt;name&gt; element tag for the domain &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct Domain {
    /// The hosts attribute. Default value is "all"
    hosts: String,
    /// The name of the domain
    #[serde(rename = "$value")]
    name: String,
}

/// Type for &lt;name&gt; element under the domain &lt;info&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainInfoRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    xmlns: String,
    /// The data for the domain to be queried
    #[serde(rename = "domain:name", alias = "name")]
    domain: Domain,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    auth_info: Option<DomainAuthInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;info&gt; command for domains
pub struct DomainInfo {
    /// The data under the &lt;info&gt; tag for domain info
    #[serde(rename = "domain:info", alias = "info")]
    info: DomainInfoRequestData,
}

// Response

/// The two types of ns lists, hostObj and hostAttr, that may be returned in the
/// domain info response
#[derive(Deserialize, Debug)]
pub struct DomainNsList {
    /// List of &lt;hostObj&gt; ns elements
    #[serde(rename = "hostObj")]
    pub host_obj: Option<Vec<StringValue>>,
    /// List of &lt;hostAttr&gt; ns elements
    pub host_attr: Option<Vec<HostAttr>>,
}

/// Type that represents the &lt;infData&gt; tag for domain info response
#[derive(Deserialize, Debug)]
pub struct DomainInfoResponseData {
    /// The domain name
    pub name: StringValue,
    /// The domain ROID
    pub roid: StringValue,
    /// The list of domain statuses
    #[serde(rename = "status")]
    pub statuses: Option<Vec<DomainStatus>>,
    /// The domain registrant
    pub registrant: Option<StringValue>,
    /// The list of domain contacts
    #[serde(rename = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of domain nameservers
    #[serde(rename = "ns")]
    pub ns: Option<DomainNsList>,
    /// The list of domain hosts
    #[serde(rename = "host")]
    pub hosts: Option<Vec<StringValue>>,
    /// The epp user who owns the domain
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// The epp user who created the domain
    #[serde(rename = "crID")]
    pub creator_id: Option<StringValue>,
    /// The domain creation date
    #[serde(rename = "crDate")]
    pub created_at: Option<StringValue>,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: Option<StringValue>,
    /// The epp user who last updated the domain
    #[serde(rename = "upID")]
    pub updater_id: Option<StringValue>,
    /// The domain last updated date
    #[serde(rename = "upDate")]
    pub updated_at: Option<StringValue>,
    /// The domain transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    /// The domain auth info
    #[serde(rename = "authInfo")]
    pub auth_info: Option<DomainAuthInfo>,
}

/// Type that represents the &lt;resData&gt; tag for domain info response
#[derive(Deserialize, Debug)]
pub struct DomainInfoResponse {
    /// Data under the &lt;resData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: DomainInfoResponseData,
}

#[cfg(test)]
mod tests {
    use super::DomainInfo;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/domain/info.xml").unwrap();

        let object = DomainInfo::new("eppdev.com", Some("2fooBAR"));

        let serialized =
            <DomainInfo as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/info.xml").unwrap();
        let object =
            <DomainInfo as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();
        let auth_info = result.info_data.auth_info.as_ref().unwrap();
        let ns_list = result.info_data.ns.as_ref().unwrap();
        let ns = (*ns_list).host_obj.as_ref().unwrap();
        let hosts = result.info_data.hosts.as_ref().unwrap();
        let statuses = result.info_data.statuses.as_ref().unwrap();
        let registrant = result.info_data.registrant.as_ref().unwrap();
        let contacts = result.info_data.contacts.as_ref().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.info_data.name, "eppdev-1.com".into());
        assert_eq!(result.info_data.roid, "125899511_DOMAIN_COM-VRSN".into());
        assert_eq!(statuses[0].status, "ok".to_string());
        assert_eq!(statuses[1].status, "clientTransferProhibited".to_string());
        assert_eq!(*registrant, "eppdev-contact-2".into());
        assert_eq!(contacts[0].id, "eppdev-contact-2".to_string());
        assert_eq!(contacts[0].contact_type, "admin".to_string());
        assert_eq!(contacts[1].id, "eppdev-contact-2".to_string());
        assert_eq!(contacts[1].contact_type, "tech".to_string());
        assert_eq!(contacts[2].id, "eppdev-contact-2".to_string());
        assert_eq!(contacts[2].contact_type, "billing".to_string());
        assert_eq!((*ns)[0], "ns1.eppdev-1.com".into());
        assert_eq!((*ns)[1], "ns2.eppdev-1.com".into());
        assert_eq!((*hosts)[0], "ns1.eppdev-1.com".into());
        assert_eq!((*hosts)[1], "ns2.eppdev-1.com".into());
        assert_eq!(result.info_data.client_id, "eppdev".into());
        assert_eq!(
            *result.info_data.creator_id.as_ref().unwrap(),
            "SYSTEM".into()
        );
        assert_eq!(
            *result.info_data.created_at.as_ref().unwrap(),
            "2021-07-23T15:31:20.0Z".into()
        );
        assert_eq!(
            *result.info_data.updater_id.as_ref().unwrap(),
            "SYSTEM".into()
        );
        assert_eq!(
            *result.info_data.updated_at.as_ref().unwrap(),
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(
            *result.info_data.expiring_at.as_ref().unwrap(),
            "2023-07-23T15:31:20.0Z".into()
        );
        assert_eq!((*auth_info).password, "epP4uthd#v".into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn response_alt() {
        let xml = get_xml("response/domain/info_alt.xml").unwrap();
        <DomainInfo as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();
    }
}
