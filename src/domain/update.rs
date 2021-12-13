//! Types for EPP domain check request
//!
use super::{DomainAuthInfo, DomainContact, HostList, XMLNS};
use crate::{
    common::{NoExtension, ObjectStatus, StringValue},
    request::{Command, Transaction},
};

use serde::Serialize;

impl Transaction<NoExtension> for DomainUpdate {}

impl Command for DomainUpdate {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl DomainUpdate {
    pub fn new(name: &str) -> Self {
        Self {
            domain: DomainUpdateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo) {
        self.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove) {
        self.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove) {
        self.domain.remove = Some(remove);
    }
}

/// Type for elements under the &lt;chg&gt; tag for domain update
#[derive(Serialize, Debug)]
pub struct DomainChangeInfo {
    /// The new registrant contact for the domain
    #[serde(rename = "domain:registrant", alias = "update")]
    pub registrant: Option<StringValue>,
    /// The new auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    pub auth_info: Option<DomainAuthInfo>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Serialize, Debug)]
pub struct DomainAddRemove {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    pub ns: Option<HostList>,
    /// The list of contacts to add to or remove from the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The list of statuses to add to or remove from the domain
    #[serde(rename = "domain:status", alias = "status")]
    pub statuses: Option<Vec<ObjectStatus>>,
}

/// Type for elements under the &lt;update&gt; tag for domain update
#[derive(Serialize, Debug)]
pub struct DomainUpdateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    pub xmlns: String,
    /// The name of the domain to update
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    #[serde(rename = "domain:add", alias = "add")]
    pub add: Option<DomainAddRemove>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    #[serde(rename = "domain:rem", alias = "rem")]
    pub remove: Option<DomainAddRemove>,
    /// The data under the &lt;chg&gt; tag for domain update
    #[serde(rename = "domain:chg", alias = "chg")]
    pub change_info: Option<DomainChangeInfo>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;update&gt; command for domains
pub struct DomainUpdate {
    #[serde(rename = "domain:update", alias = "update")]
    pub domain: DomainUpdateRequestData,
}

#[cfg(test)]
mod tests {
    use super::{DomainAddRemove, DomainAuthInfo, DomainChangeInfo, DomainContact, DomainUpdate};
    use crate::common::{NoExtension, ObjectStatus};
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/domain/update.xml").unwrap();

        let mut object = DomainUpdate::new("eppdev.com");

        let add = DomainAddRemove {
            ns: None,
            contacts: None,
            statuses: Some(vec![ObjectStatus {
                status: "clientDeleteProhibited".to_string(),
            }]),
        };

        let remove = DomainAddRemove {
            ns: None,
            contacts: Some(vec![DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-2".to_string(),
            }]),
            statuses: None,
        };

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: Some(DomainAuthInfo::new("epP5uthd#v")),
        };

        object.add(add);
        object.remove(remove);
        object.info(change_info);

        let serialized =
            <DomainUpdate as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/update.xml").unwrap();
        let object =
            <DomainUpdate as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
