use std::fmt::Debug;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::common::{Options, ServiceExtension, Services, StringValue, EPP_XMLNS};

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

// Response

/// Type for data within the <svcMenu> section of an EPP greeting
#[derive(Debug, Eq, PartialEq)]
pub struct ServiceMenu {
    pub options: Options<'static>,
    pub services: Services<'static>,
}

/// Simplified service menu type for deserialization to `ServiceMenu` type from EPP greeting XML
#[derive(Deserialize, Debug, PartialEq)]
struct FlattenedServiceMenu {
    pub version: StringValue<'static>,
    pub lang: StringValue<'static>,
    #[serde(rename = "objURI")]
    pub obj_uris: Vec<StringValue<'static>>,
    #[serde(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension<'static>>,
}

impl<'a, 'de: 'a> Deserialize<'de> for ServiceMenu {
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
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct All;

/// Type corresponding to <none> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct NoAccess;

/// Type corresponding to <null> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Null;

/// Type corresponding to <personal> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Personal;

/// Type corresponding to <personalAndOther> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct PersonalAndOther;

/// Type corresponding to <other> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Other;

/// Type corresponding to possible <retention> type values
#[derive(Deserialize, Debug, Eq, PartialEq)]
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
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Access {
    #[serde(flatten)]
    pub ty: AccessType,
}

/// Type corresponding to possible <purpose> type values
#[derive(Deserialize, Debug, Eq, PartialEq)]
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
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Purpose {
    #[serde(rename = "$value")]
    pub purpose: Vec<PurposeType>,
}

/// Type corresponding to possible <purpose> type values
#[derive(Deserialize, Debug, Eq, PartialEq)]
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
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Recipient {
    #[serde(rename = "$value")]
    pub recipient: Vec<RecipientType>,
}

/// Type corresponding to <business> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Business;

/// Type corresponding to <indefinite> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Indefinite;

/// Type corresponding to <legal> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Legal;

/// Type corresponding to <none> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct No;

/// Type corresponding to <stated> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Stated;

/// Type corresponding to possible <retention> type values
#[derive(Deserialize, Debug, Eq, PartialEq)]
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
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Retention {
    #[serde(flatten)]
    pub ty: RetentionType,
}

/// Type corresponding to <statement> in the EPP greeting XML (pending more compliant implementation)
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Statement {
    /// Data for the <purpose> tag
    pub purpose: Purpose,
    /// Data for the <recipient> tag
    pub recipient: Recipient,
    /// Data for the <retention> tag
    pub retention: Retention,
}

/// Type corresponding to <absolute> value in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Absolute {
    #[serde(rename = "$value")]
    pub absolute: StringValue<'static>,
}

/// Type corresponding to <relative> value in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Relative {
    #[serde(rename = "$value")]
    pub relative: StringValue<'static>,
}

/// Type corresponding to possible <expiry> type values
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub enum ExpiryType {
    /// Data for the <absolute> tag
    #[serde(rename = "absolute")]
    Absolute(Absolute),
    /// Data for the <relative> tag
    #[serde(rename = "relative")]
    Relative(Relative),
}

/// Type corresponding to <expiry> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Expiry {
    #[serde(flatten)]
    pub ty: ExpiryType,
}

/// Type corresponding to <dcp> in the EPP greeting XML
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Dcp {
    /// Data for the <access> tag
    pub access: Access,
    /// Data for the <statement> tags
    pub statement: Vec<Statement>,
    /// Data for the <expiry> tag
    pub expiry: Option<Expiry>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
/// Type corresponding to the <greeting> tag in the EPP greeting XML
pub struct Greeting {
    /// The service ID
    #[serde(rename = "svID")]
    pub service_id: String,
    /// The date from the EPP server
    #[serde(rename = "svDate")]
    pub service_date: DateTime<Utc>,
    /// Data under the <svcMenu> element
    #[serde(rename = "svcMenu")]
    pub svc_menu: ServiceMenu,
    /// Data under the <dcp> element
    pub dcp: Dcp,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename = "epp")]
pub struct GreetingDocument {
    #[serde(rename = "greeting")]
    pub data: Greeting,
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::{ExpiryType, GreetingDocument, HelloDocument, Relative};
    use crate::tests::get_xml;
    use crate::xml;

    #[test]
    fn hello() {
        let xml = get_xml("request/hello.xml").unwrap();
        let serialized = xml::serialize(&HelloDocument::default()).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn greeting() {
        let xml = get_xml("response/greeting.xml").unwrap();
        let object = xml::deserialize::<GreetingDocument>(xml.as_str()).unwrap();

        assert_eq!(object.data.service_id, "ISPAPI EPP Server");
        assert_eq!(
            object.data.service_date,
            Utc.with_ymd_and_hms(2021, 7, 25, 14, 51, 17).unwrap()
        );
        assert_eq!(object.data.svc_menu.options.version, "1.0".into());
        assert_eq!(object.data.svc_menu.options.lang, "en".into());
        assert_eq!(object.data.svc_menu.services.obj_uris.len(), 4);
        assert_eq!(
            object
                .data
                .svc_menu
                .services
                .svc_ext
                .unwrap()
                .ext_uris
                .unwrap()
                .len(),
            5
        );
        assert_eq!(object.data.dcp.statement.len(), 2);
        assert_eq!(
            object.data.dcp.expiry.unwrap().ty,
            ExpiryType::Relative(Relative {
                relative: "P1M".into()
            })
        );
    }
}
