//! Types for EPP domain create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{
    DomainAuthInfo, DomainContact, ElementName, HostList, NoExtension, Period, StringValue,
};
use crate::request::{EppExtension, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DomainCreate<E> {
    request: DomainCreateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for DomainCreate<E> {
    type Input = DomainCreateRequest;
    type Output = DomainCreateResponse;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> DomainCreate<E> {
    pub fn new(
        name: &str,
        period: u16,
        ns: Option<HostList>,
        registrant_id: Option<&str>,
        auth_password: &str,
        contacts: Option<Vec<DomainContact>>,
    ) -> DomainCreate<NoExtension> {
        let registrant = registrant_id.map(|id| id.into());
        let domain_create = DomainCreateRequest {
            domain: DomainCreateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                period: Period::new(period),
                ns,
                registrant,
                auth_info: DomainAuthInfo::new(auth_password),
                contacts,
            },
        };

        DomainCreate {
            request: domain_create,
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> DomainCreate<F> {
        DomainCreate {
            request: self.request,
            extension: Some(extension),
        }
    }
}

// Request

/// Type for elements under the domain &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateRequestData {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain", alias = "xmlns")]
    pub xmlns: String,
    /// The domain name
    #[serde(rename = "domain:name", alias = "name")]
    pub name: StringValue,
    /// The period of registration
    #[serde(rename = "domain:period", alias = "period")]
    pub period: Period,
    /// The list of nameserver hosts
    /// either of type `HostObjList` or `HostAttrList`
    #[serde(rename = "domain:ns", alias = "ns")]
    pub ns: Option<HostList>,
    /// The domain registrant
    #[serde(rename = "domain:registrant", alias = "registrant")]
    pub registrant: Option<StringValue>,
    /// The list of contacts for the domain
    #[serde(rename = "domain:contact", alias = "contact")]
    pub contacts: Option<Vec<DomainContact>>,
    /// The auth info for the domain
    #[serde(rename = "domain:authInfo", alias = "authInfo")]
    pub auth_info: DomainAuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for domains
pub struct DomainCreateRequest {
    /// The data for the domain to be created with
    /// T being the type of nameserver list (`HostObjList` or `HostAttrList`)
    /// to be supplied
    #[serde(rename = "domain:create", alias = "create")]
    pub domain: DomainCreateRequestData,
}

// Response

/// Type that represents the &lt;chkData&gt; tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
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
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for domain create response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainCreateResponse {
    /// Data under the &lt;chkData&gt; tag
    #[serde(rename = "creData")]
    pub create_data: DomainCreateResponseData,
}
