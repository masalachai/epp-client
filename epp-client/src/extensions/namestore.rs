//! Types for EPP namestore request and responses

use serde::{Deserialize, Serialize};

use crate::common::StringValue;
use crate::domain::check::DomainCheck;
use crate::request::{Extension, Transaction};

pub const XMLNS: &str = "http://www.verisign-grs.com/epp/namestoreExt-1.1";

impl Transaction<NameStore> for DomainCheck {}

impl NameStore {
    /// Create a new RGP restore report request
    pub fn new(subproduct: &str) -> NameStore {
        NameStore {
            data: NameStoreData {
                xmlns: XMLNS.to_string(),
                subproduct: subproduct.into(),
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
    pub subproduct: StringValue,
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
            object,
            Some(namestore_ext),
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
