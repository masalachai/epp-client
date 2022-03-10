//! Types for EPP domain check request
//!
use super::{DomainAuthInfo, DomainContact, HostList, XMLNS};
use crate::{
    common::{NoExtension, ObjectStatus, StringValue},
    request::{Command, Transaction},
};

use serde::Serialize;

impl<'a> Transaction<NoExtension> for DomainUpdate<'a> {}

impl<'a> Command for DomainUpdate<'a> {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl<'a> DomainUpdate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            domain: DomainUpdateRequestData {
                xmlns: XMLNS,
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; tag
    pub fn info(&mut self, info: DomainChangeInfo<'a>) {
        self.domain.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; tag
    pub fn add(&mut self, add: DomainAddRemove<'a>) {
        self.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainAddRemove<'a>) {
        self.domain.remove = Some(remove);
    }
}

/// Type for elements under the &lt;chg&gt; tag for domain update
#[derive(Serialize, Debug)]
pub struct DomainChangeInfo<'a> {
    /// The new registrant contact for the domain
    #[serde(rename = "domain:registrant")]
    pub registrant: Option<StringValue<'a>>,
    /// The new auth info for the domain
    #[serde(rename = "domain:authInfo")]
    pub auth_info: Option<DomainAuthInfo<'a>>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Serialize, Debug)]
pub struct DomainAddRemove<'a> {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns")]
    pub ns: Option<HostList<'a>>,
    /// The list of contacts to add to or remove from the domain
    #[serde(rename = "domain:contact")]
    pub contacts: Option<&'a [DomainContact<'a>]>,
    /// The list of statuses to add to or remove from the domain
    #[serde(rename = "domain:status")]
    pub statuses: Option<&'a [ObjectStatus<'a>]>,
}

/// Type for elements under the &lt;update&gt; tag for domain update
#[derive(Serialize, Debug)]
pub struct DomainUpdateRequestData<'a> {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    pub xmlns: &'a str,
    /// The name of the domain to update
    #[serde(rename = "domain:name")]
    pub name: StringValue<'a>,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    #[serde(rename = "domain:add")]
    pub add: Option<DomainAddRemove<'a>>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    #[serde(rename = "domain:rem")]
    pub remove: Option<DomainAddRemove<'a>>,
    /// The data under the &lt;chg&gt; tag for domain update
    #[serde(rename = "domain:chg")]
    pub change_info: Option<DomainChangeInfo<'a>>,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;update&gt; command for domains
pub struct DomainUpdate<'a> {
    #[serde(rename = "domain:update")]
    pub domain: DomainUpdateRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::{DomainAddRemove, DomainAuthInfo, DomainChangeInfo, DomainContact, DomainUpdate};
    use crate::common::{NoExtension, ObjectStatus};
    use crate::request::Transaction;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, get_xml, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let mut object = DomainUpdate::new("eppdev.com");

        let statuses = &[ObjectStatus {
            status: "clientDeleteProhibited".into(),
        }];

        let add = DomainAddRemove {
            ns: None,
            contacts: None,
            statuses: Some(statuses),
        };

        let contacts = &[DomainContact {
            contact_type: "billing".into(),
            id: "eppdev-contact-2".into(),
        }];

        let remove = DomainAddRemove {
            ns: None,
            contacts: Some(contacts),
            statuses: None,
        };

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: Some(DomainAuthInfo::new("epP5uthd#v")),
        };

        object.add(add);
        object.remove(remove);
        object.info(change_info);
        assert_serialized("request/domain/update.xml", &object);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/update.xml").unwrap();
        let object =
            <DomainUpdate as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
