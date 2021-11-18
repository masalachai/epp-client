//! Types for EPP domain info response

use serde::{Deserialize, Serialize};

use crate::epp::object::data::{AuthInfo, DomainContact, DomainStatus, HostAttr};
use crate::epp::object::{EppObject, StringValue};
use crate::epp::response::domain::rgp::request::RgpRequestResult;
use crate::epp::response::CommandResponseWithExtension;

/// Type that represents the &lt;epp&gt; tag for the EPP XML domain info response
pub type EppDomainInfoResponse =
    EppObject<CommandResponseWithExtension<DomainInfoResult, RgpRequestResult>>;

/// The two types of ns lists, hostObj and hostAttr, that may be returned in the
/// domain info response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainNsList {
    /// List of &lt;hostObj&gt; ns elements
    #[serde(rename = "hostObj")]
    pub host_obj: Option<Vec<StringValue>>,
    /// List of &lt;hostAttr&gt; ns elements
    pub host_attr: Option<Vec<HostAttr>>,
}

/// Type that represents the &lt;infData&gt; tag for domain info response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoData {
    /// XML namespace for domain response data
    #[serde(rename = "xmlns:domain")]
    xmlns: String,
    /// The domain name
    pub name: StringValue,
    /// The domain ROID
    pub roid: StringValue,
    /// The list of domain statuses
    #[serde(rename = "status")]
    pub statuses: Vec<DomainStatus>,
    /// The domain registrant
    pub registrant: StringValue,
    /// The list of domain contacts
    #[serde(rename = "contact")]
    pub contacts: Vec<DomainContact>,
    /// The list of domain nameservers
    #[serde(rename = "ns")]
    pub ns: Option<DomainNsList>,
    /// The list of domain hosts
    #[serde(rename = "host")]
    pub hosts: Option<Vec<StringValue>>,
    /// The epp user who owns the domain
    #[serde(rename = "clID")]
    pub client_id: StringValue,
    /// The epp user who created the domain
    #[serde(rename = "crID")]
    pub creator_id: StringValue,
    /// The domain creation date
    #[serde(rename = "crDate")]
    pub created_at: StringValue,
    /// The epp user who last updated the domain
    #[serde(rename = "upID")]
    pub updater_id: StringValue,
    /// The domain last updated date
    #[serde(rename = "upDate")]
    pub updated_at: StringValue,
    /// The domain expiry date
    #[serde(rename = "exDate")]
    pub expiring_at: StringValue,
    /// The domain transfer date
    #[serde(rename = "trDate")]
    pub transferred_at: Option<StringValue>,
    /// The domain auth info
    #[serde(rename = "authInfo")]
    pub auth_info: Option<AuthInfo>,
}

/// Type that represents the &lt;resData&gt; tag for domain info response
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfoResult {
    /// Data under the &lt;resData&gt; tag
    #[serde(rename = "infData")]
    pub info_data: DomainInfoData,
}
