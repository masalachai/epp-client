//! Types for EPP namestore request and responses

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

impl<'a> Transaction<NameStore> for ContactCheck<'a> {}
impl<'a> Transaction<NameStore> for ContactCreate<'a> {}
impl<'a> Transaction<NameStore> for ContactDelete<'a> {}
impl<'a> Transaction<NameStore> for ContactInfo<'a> {}
impl<'a> Transaction<NameStore> for ContactUpdate<'a> {}

// Domain

impl<'a> Transaction<NameStore> for DomainCheck<'a> {}
impl<'a> Transaction<NameStore> for DomainCreate<'a> {}
impl<'a> Transaction<NameStore> for DomainDelete<'a> {}
impl<'a> Transaction<NameStore> for DomainInfo<'a> {}
impl<'a> Transaction<NameStore> for DomainRenew<'a> {}
impl<'a> Transaction<NameStore> for DomainTransfer<'a> {}
impl<'a> Transaction<NameStore> for DomainUpdate<'a> {}

// Host

impl<'a> Transaction<NameStore> for HostCheck<'a> {}
impl<'a> Transaction<NameStore> for HostCreate<'a> {}
impl<'a> Transaction<NameStore> for HostDelete<'a> {}
impl<'a> Transaction<NameStore> for HostInfo<'a> {}
impl<'a> Transaction<NameStore> for HostUpdate<'a> {}

impl NameStore {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            data: NameStoreData {
                xmlns: XMLNS.to_string(),
                subproduct: subproduct.to_owned().into(),
            },
        }
    }
}

impl Extension for NameStore {
    type Response = NameStore;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "namestoreExt:namestoreExt")]
pub struct NameStore {
    #[serde(rename = "namestoreExt:namestoreExt", alias = "namestoreExt")]
    pub data: NameStoreData,
}

#[derive(Serialize, Deserialize, Debug)]
/// Type for EPP XML &lt;namestoreExt&gt; extension
pub struct NameStoreData {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:namestoreExt", alias = "xmlns")]
    pub xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "namestoreExt:subProduct", alias = "subProduct")]
    pub subproduct: StringValue<'static>,
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
