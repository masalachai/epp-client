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

impl Transaction<NameStore<'_>> for ContactCheck<'_> {}
impl Transaction<NameStore<'_>> for ContactCreate<'_> {}
impl Transaction<NameStore<'_>> for ContactDelete<'_> {}
impl Transaction<NameStore<'_>> for ContactInfo<'_> {}
impl Transaction<NameStore<'_>> for ContactUpdate<'_> {}

// Domain

impl Transaction<NameStore<'_>> for DomainCheck<'_> {}
impl Transaction<NameStore<'_>> for DomainCreate<'_> {}
impl Transaction<NameStore<'_>> for DomainDelete<'_> {}
impl Transaction<NameStore<'_>> for DomainInfo<'_> {}
impl Transaction<NameStore<'_>> for DomainRenew<'_> {}
impl Transaction<NameStore<'_>> for DomainTransfer<'_> {}
impl Transaction<NameStore<'_>> for DomainUpdate<'_> {}

// Host

impl Transaction<NameStore<'_>> for HostCheck<'_> {}
impl Transaction<NameStore<'_>> for HostCreate<'_> {}
impl Transaction<NameStore<'_>> for HostDelete<'_> {}
impl Transaction<NameStore<'_>> for HostInfo<'_> {}
impl Transaction<NameStore<'_>> for HostUpdate<'_> {}

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
    use crate::tests::{assert_serialized, get_xml};

    #[test]
    fn command() {
        let namestore_ext = NameStore::new("com");

        let object = DomainCheck {
            domains: &["example1.com", "example2.com", "example3.com"],
        };

        assert_serialized(
            "request/extensions/namestore.xml",
            (&object, &namestore_ext),
        );
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
