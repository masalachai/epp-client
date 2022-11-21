//! Types for EPP namestore request and responses

use std::borrow::Cow;

use instant_xml::{FromXml, ToXml};

use crate::{
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
    pub fn new(subproduct: &'a str) -> NameStore {
        NameStore {
            subproduct: subproduct.into(),
        }
    }
}

impl<'a> Extension for NameStore<'a> {
    type Response = NameStore<'static>;
}

#[derive(Debug, FromXml, ToXml)]
/// Type for EPP XML &lt;namestoreExt&gt; extension
#[xml(rename = "namestoreExt", ns(XMLNS))]
pub struct NameStore<'a> {
    /// The object holding the list of domains to be checked
    #[xml(rename = "subProduct")]
    pub subproduct: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
    use super::NameStore;
    use crate::domain::check::DomainCheck;
    use crate::tests::{assert_serialized, response_from_file_with_ext};

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
        let object = response_from_file_with_ext::<DomainCheck, NameStore>(
            "response/extensions/namestore.xml",
        );
        let ext = object.extension().unwrap();
        assert_eq!(ext.subproduct, "com");
    }
}
