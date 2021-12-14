//! Types for EPP domain create request

use super::{DomainAuthInfo, DomainContact, HostList, Period, XMLNS};
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};

use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for DomainCreate {}

impl Command for DomainCreate {
    type Response = DomainCreateResponse;
    const COMMAND: &'static str = "create";
}

// Request

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainCreateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    pub xmlns: String,
    /// The domain name
    #[serde(rename = "domain:name")]
    pub name: StringValue,
    /// The period of registration
    #[serde(rename = "domain:period")]
    pub period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns")]
    pub ns: Option<HostList>,
    /// The domain registrant
    #[serde(rename = "domain:registrant")]
    pub registrant: Option<StringValue>,
    /// The list of contacts for the domain
    #[serde(rename = "domain:contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo")]
    pub auth_info: DomainAuthInfo,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;create&gt; command for domains
pub struct DomainCreate {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "domain:create")]
    pub domain: DomainCreateRequestData,
}

impl DomainCreate {
    pub fn new(
        name: &str,
        period: u16,
        ns: Option<HostList>,
        registrant_id: Option<&str>,
        auth_password: &str,
        contacts: Option<Vec<DomainContact>>,
    ) -> Self {
        Self {
            domain: DomainCreateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns,
                registrant: registrant_id.map(|id| id.into()),
                auth_info: DomainAuthInfo::new(auth_password),
                contacts,
            },
        }
    }
}

// Response

/// Type that represents the &lt;chkData&gt; tag for domain create response
#[derive(Deserialize, Debug)]
pub struct DomainCreateResponseData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    pub xmlns: String,
    /// The domain name
    pub name: StringValue,
    /// The creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: Option<StringValue>,
}

/// Type that represents the &lt;resData&gt; tag for domain create response
#[derive(Deserialize, Debug)]
pub struct DomainCreateResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: DomainCreateResponseData,
}

#[cfg(test)]
mod tests {
    use super::{DomainContact, DomainCreate, HostList};
    use crate::common::{HostAddr, NoExtension};
    use crate::domain::{HostAttr, HostAttrList, HostObjList};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/domain/create.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let object = DomainCreate::new(
            "eppdev-1.com",
            1,
            None,
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        let serialized =
            <DomainCreate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn command_with_host_obj() {
        let xml = get_xml("request/domain/create_with_host_obj.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let ns = Some(HostList::HostObjList(HostObjList {
            hosts: vec!["ns1.test.com".into(), "ns2.test.com".into()],
        }));

        let object = DomainCreate::new(
            "eppdev-1.com",
            1,
            ns,
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        let serialized =
            <DomainCreate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn command_with_host_attr() {
        let xml = get_xml("request/domain/create_with_host_attr.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let host_attr = HostList::HostAttrList(HostAttrList {
            hosts: vec![
                HostAttr {
                    name: "ns1.eppdev-1.com".into(),
                    addresses: None,
                },
                HostAttr {
                    name: "ns2.eppdev-1.com".into(),
                    addresses: Some(vec![
                        HostAddr::new_v4("177.232.12.58"),
                        HostAddr::new_v6("2404:6800:4001:801::200e"),
                    ]),
                },
            ],
        });

        let object = DomainCreate::new(
            "eppdev-2.com",
            1,
            Some(host_attr),
            Some("eppdev-contact-3"),
            "epP4uthd#v",
            Some(contacts),
        );

        let serialized =
            <DomainCreate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/create.xml").unwrap();
        let object =
            <DomainCreate as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.create_data.name, "eppdev-2.com".into());
        assert_eq!(
            result.create_data.created_at,
            "2021-07-25T18:11:35.0Z".into()
        );
        assert_eq!(
            *result.create_data.expiring_at.as_ref().unwrap(),
            "2022-07-25T18:11:34.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
