//! Types for EPP namestore request and responses

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{
    common::StringValue,
    contact::{
        check::ContactCheck, create::ContactCreate, delete::ContactDelete, info::ContactInfo,
        update::ContactUpdate,
    },
    domain::{
        check::DomainCheck, create::DomainCreate, delete::DomainDelete, info::DomainInfo,
        renew::DomainRenew, transfer::DomainTransfer, update::DomainUpdate,
    },
    host::{
        check::HostCheck, create::HostCreate, delete::HostDelete, info::HostInfo,
        update::HostUpdate,
    },
    request::{Extension, Transaction},
};

pub const XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

// Contact

impl<'a> Transaction<NameStore<'a>> for ContactCheck<'a> {}
impl<'a> Transaction<NameStore<'a>> for ContactCreate<'a> {}
impl<'a> Transaction<NameStore<'a>> for ContactDelete<'a> {}
impl<'a> Transaction<NameStore<'a>> for ContactInfo<'a> {}
impl<'a> Transaction<NameStore<'a>> for ContactUpdate<'a> {}

// Domain

impl<'a> Transaction<NameStore<'a>> for DomainCheck<'a> {}
impl<'a> Transaction<NameStore<'a>> for DomainCreate<'a> {}
impl<'a> Transaction<NameStore<'a>> for DomainDelete<'a> {}
impl<'a> Transaction<NameStore<'a>> for DomainInfo<'a> {}
impl<'a> Transaction<NameStore<'a>> for DomainRenew<'a> {}
impl<'a> Transaction<NameStore<'a>> for DomainTransfer<'a> {}
impl<'a> Transaction<NameStore<'a>> for DomainUpdate<'a> {}

// Host

impl<'a> Transaction<NameStore<'a>> for HostCheck<'a> {}
impl<'a> Transaction<NameStore<'a>> for HostCreate<'a> {}
impl<'a> Transaction<NameStore<'a>> for HostDelete<'a> {}
impl<'a> Transaction<NameStore<'a>> for HostInfo<'a> {}
impl<'a> Transaction<NameStore<'a>> for HostUpdate<'a> {}

impl<'a> NameStore<'a> {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            data: NameStoreData {
                xmlns: XMLNS.into(),
                subproduct: subproduct.to_owned().into(),
            },
        }
    }
}

impl<'a> NameStoreData<'a> {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> Self {
        Self {
            xmlns: XMLNS.into(),
            subproduct: subproduct.to_owned().into(),
        }
    }
}

impl<'a> Extension for NameStore<'a> {
    type Response = NameStore<'static>;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "namestoreExt:namestoreExt")]
pub struct NameStore<'a> {
    #[serde(rename = "namestoreExt:namestoreExt", alias = "namestoreExt")]
    pub data: NameStoreData<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Type for EPP XML &lt;namestoreExt&gt; extension
pub struct NameStoreData<'a> {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:namestoreExt", alias = "xmlns")]
    pub xmlns: Cow<'a, str>,
    /// The object holding the list of domains to be checked
    #[serde(rename = "namestoreExt:subProduct", alias = "subProduct")]
    pub subproduct: StringValue<'a>,
}

#[cfg(test)]
mod tests {
    use super::NameStore;
    use crate::domain::check::DomainCheck;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID};

    #[test]
    fn command() {
        let xml = get_xml("request/extensions/namestore.xml").unwrap();

        let namestore_ext = NameStore::new("com");

        let object = DomainCheck::new(vec!["example1.com", "example2.com", "example3.com"]);

        let serialized = <DomainCheck as Transaction<NameStore>>::serialize_request(
            &object,
            Some(&namestore_ext),
            CLTRID,
        )
        .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/extensions/namestore.xml").unwrap();

        let object =
            <DomainCheck as Transaction<NameStore>>::deserialize_response(xml.as_str()).unwrap();

        let ext = object.extension.unwrap();

        assert_eq!(ext.data.subproduct, "com".into());
    }
}
