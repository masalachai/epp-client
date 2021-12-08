use std::fmt::Debug;

use epp_client_macros::ElementName;
use serde::{Deserialize, Deserializer, Serialize};

use crate::common::{ElementName, Options, ServiceExtension, Services, StringValue, EPP_XMLNS};
use crate::xml::EppXml;

// Request

#[derive(Debug, PartialEq, Serialize)]
struct Hello;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename = "epp")]
pub struct HelloDocument {
    xmlns: &'static str,
    hello: Hello,
}

impl Default for HelloDocument {
    fn default() -> Self {
        Self {
            xmlns: EPP_XMLNS,
            hello: Hello,
        }
    }
}

impl EppXml for HelloDocument {}

// Response

/// Type for data within the <svcMenu> section of an EPP greeting
#[derive(Serialize, Debug, PartialEq)]
pub struct ServiceMenu {
    pub options: Options,
    pub services: Services,
}

/// Simplified service menu type for deserialization to `ServiceMenu` type from EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FlattenedServiceMenu {
    pub version: StringValue,
    pub lang: StringValue,
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue>,
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension>,
}

impl<'de> Deserialize<'de> for ServiceMenu {
    /// Deserializes the <svcMenu> data to the `ServiceMenu` type
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let flattened_svc_menu = FlattenedServiceMenu::deserialize(deserializer)?;

        let svc_menu = ServiceMenu {
            options: Options {
                version: flattened_svc_menu.version,
                lang: flattened_svc_menu.lang,
            },
            services: Services {
                obj_uris: flattened_svc_menu.obj_uris,
                svc_ext: flattened_svc_menu.svc_ext,
            },
        };

        Ok(svc_menu)
    }
}

/// Type corresponding to <all> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct All;

/// Type corresponding to <none> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NoAccess;

/// Type corresponding to <null> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Null;

/// Type corresponding to <personal> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Personal;

/// Type corresponding to <personalAndOther> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PersonalAndOther;

/// Type corresponding to <other> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Other;

/// Type corresponding to possible <retention> type values
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AccessType {
    /// Data for the <all> tag
    #[serde(rename = "all")]
    All(All),
    /// Data for the <none> tag
    #[serde(rename = "none")]
    NoAccess(NoAccess),
    /// Data for the <null> tag
    #[serde(rename = "null")]
    Null(Null),
    /// Data for the <personal> tag
    #[serde(rename = "personal")]
    Personal(Personal),
    /// Data for the <personalAndOther> tag
    #[serde(rename = "personalAndOther")]
    PersonalAndOther(PersonalAndOther),
    /// Data for the <other> tag
    #[serde(rename = "other")]
    Other(Other),
}

/// Type corresponding to <access> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Access {
    #[serde(flatten)]
    pub ty: AccessType,
}

/// Type corresponding to possible <purpose> type values
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PurposeType {
    /// Data for the <admin> tag
    #[serde(rename = "admin")]
    Admin,
    /// Data for the <contact> tag
    #[serde(rename = "contact")]
    Contact,
    /// Data for the <prov> tag
    #[serde(rename = "prov")]
    Prov,
    /// Data for the <other> tag
    #[serde(rename = "other")]
    OtherPurpose,
}

/// Type corresponding to <purpose> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Purpose {
    #[serde(rename = "$value")]
    pub purpose: Vec<PurposeType>,
}

/// Type corresponding to possible <purpose> type values
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RecipientType {
    /// Data for the <other> tag
    #[serde(rename = "other")]
    Other,
    /// Data for the <ours> tag
    #[serde(rename = "ours")]
    Ours,
    /// Data for the <public> tag
    #[serde(rename = "public")]
    Public,
    /// Data for the <same> tag
    #[serde(rename = "same")]
    Same,
    /// Data for the <unrelated> tag
    #[serde(rename = "unrelated")]
    Unrelated,
}

/// Type corresponding to <recipeint> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipient {
    #[serde(rename = "$value")]
    pub recipient: Vec<RecipientType>,
}

/// Type corresponding to <business> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Business;

/// Type corresponding to <indefinite> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Indefinite;

/// Type corresponding to <legal> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Legal;

/// Type corresponding to <none> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct No;

/// Type corresponding to <stated> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stated;

/// Type corresponding to possible <retention> type values
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum RetentionType {
    /// Data for the <business> tag
    #[serde(rename = "business")]
    Business(Business),
    /// Data for the <indefinite> tag
    #[serde(rename = "indefinite")]
    Indefinite(Indefinite),
    /// Data for the <legal> tag
    #[serde(rename = "legal")]
    Legal(Legal),
    /// Data for the <none> tag
    #[serde(rename = "none")]
    No(No),
    /// Data for the <stated> tag
    #[serde(rename = "stated")]
    Stated(Stated),
}

/// Type corresponding to <retention> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Retention {
    #[serde(flatten)]
    pub ty: RetentionType,
}

/// Type corresponding to <statement> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Statement {
    /// Data for the <purpose> tag
    pub purpose: Purpose,
    /// Data for the <recipient> tag
    pub recipient: Recipient,
    /// Data for the <retention> tag
    pub retention: Retention,
}

/// Type corresponding to <absolute> value in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Absolute {
    #[serde(rename = "$value")]
    pub absolute: StringValue,
}

/// Type corresponding to <relative> value in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Relative {
    #[serde(rename = "$value")]
    pub relative: StringValue,
}

/// Type corresponding to possible <expiry> type values
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ExpiryType {
    /// Data for the <absolute> tag
    #[serde(rename = "absolute")]
    Absolute(Absolute),
    /// Data for the <relative> tag
    #[serde(rename = "relative")]
    Relative(Relative),
}

/// Type corresponding to <expiry> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Expiry {
    #[serde(flatten)]
    pub ty: ExpiryType,
}

/// Type corresponding to <dcp> in the EPP greeting XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dcp {
    /// Data for the <access> tag
    pub access: Access,
    /// Data for the <statement> tags
    pub statement: Vec<Statement>,
    /// Data for the <expiry> tag
    pub expiry: Option<Expiry>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[serde(rename_all = "lowercase")]
#[element_name(name = "greeting")]
/// Type corresponding to the <greeting> tag in the EPP greeting XML
pub struct Greeting {
    /// The service ID
    #[serde(rename = "svID")]
    pub service_id: String,
    /// The date from the EPP server
    #[serde(rename = "svDate")]
    pub service_date: String,
    /// Data under the <svcMenu> element
    #[serde(rename = "svcMenu")]
    pub svc_menu: ServiceMenu,
    /// Data under the <dcp> element
    pub dcp: Dcp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct GreetingDocument {
    #[serde(rename = "greeting")]
    pub data: Greeting,
}

impl EppXml for GreetingDocument {}
