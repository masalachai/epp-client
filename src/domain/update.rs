//! Types for EPP domain check request

use instant_xml::ToXml;

use super::{DomainAuthInfo, DomainContact, NameServers, Status, XMLNS};
use crate::{
    common::{NoExtension, EPP_XMLNS},
    request::{Command, Transaction},
};

impl<'a> Transaction<NoExtension> for DomainUpdate<'a> {}

impl<'a> Command for DomainUpdate<'a> {
    type Response = ();
    const COMMAND: &'static str = "update";
}

impl<'a> DomainUpdate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            domain: DomainUpdateRequestData {
                name,
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
    pub fn add(&mut self, add: DomainAdd<'a>) {
        self.domain.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; tag
    pub fn remove(&mut self, remove: DomainRemove<'a>) {
        self.domain.remove = Some(remove);
    }
}

/// Type for elements under the &lt;chg&gt; tag for domain update
#[derive(Debug, ToXml)]
#[xml(rename = "chg", ns(XMLNS))]
pub struct DomainChangeInfo<'a> {
    /// The new registrant contact for the domain
    pub registrant: Option<&'a str>,
    /// The new auth info for the domain
    pub auth_info: Option<DomainAuthInfo<'a>>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Debug, ToXml)]
#[xml(rename = "add", ns(XMLNS))]
pub struct DomainAdd<'a> {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    pub ns: Option<NameServers<'a>>,
    /// The list of contacts to add to or remove from the domain
    pub contacts: Option<&'a [DomainContact<'a>]>,
    /// The list of statuses to add to or remove from the domain
    pub statuses: Option<&'a [Status<'a>]>,
}

/// Type for elements under the &lt;add&gt; and &lt;rem&gt; tags for domain update
#[derive(Debug, ToXml)]
#[xml(rename = "rem", ns(XMLNS))]
pub struct DomainRemove<'a> {
    /// The list of nameservers to add or remove
    /// Type T can be either a `HostObjList` or `HostAttrList`
    pub ns: Option<NameServers<'a>>,
    /// The list of contacts to add to or remove from the domain
    pub contacts: Option<&'a [DomainContact<'a>]>,
    /// The list of statuses to add to or remove from the domain
    pub statuses: Option<&'a [Status<'a>]>,
}

/// Type for elements under the &lt;update&gt; tag for domain update
#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(XMLNS))]
pub struct DomainUpdateRequestData<'a> {
    /// The name of the domain to update
    pub name: &'a str,
    /// `DomainAddRemove` Object containing the list of elements to be added
    /// to the domain
    pub add: Option<DomainAdd<'a>>,
    /// `DomainAddRemove` Object containing the list of elements to be removed
    /// from the domain
    pub remove: Option<DomainRemove<'a>>,
    /// The data under the &lt;chg&gt; tag for domain update
    #[xml(rename = "domain:chg")]
    pub change_info: Option<DomainChangeInfo<'a>>,
}

/// Type for EPP XML &lt;update&gt; command for domains
#[derive(Debug, ToXml)]
#[xml(rename = "update", ns(EPP_XMLNS))]
pub struct DomainUpdate<'a> {
    pub domain: DomainUpdateRequestData<'a>,
}

#[cfg(test)]
mod tests {
    use super::{
        DomainAdd, DomainAuthInfo, DomainChangeInfo, DomainContact, DomainRemove, DomainUpdate,
    };
    use crate::domain::Status;
    use crate::response::ResultCode;
    use crate::tests::{assert_serialized, response_from_file, CLTRID, SUCCESS_MSG, SVTRID};

    #[test]
    fn command() {
        let mut object = DomainUpdate::new("eppdev.com");

        let statuses = &[Status {
            status: "clientDeleteProhibited".into(),
        }];

        let add = DomainAdd {
            ns: None,
            contacts: None,
            statuses: Some(statuses),
        };

        let contacts = &[DomainContact {
            contact_type: "billing".into(),
            id: "eppdev-contact-2".into(),
        }];

        let remove = DomainRemove {
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
        let object = response_from_file::<DomainUpdate>("response/domain/update.xml");

        assert_eq!(object.result.code, ResultCode::CommandCompletedSuccessfully);
        assert_eq!(object.result.message, SUCCESS_MSG);
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
