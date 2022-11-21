use std::fmt::Debug;

use chrono::{DateTime, Utc};
use instant_xml::{Deserializer, FromXml, ToXml};

use crate::common::{Options, ServiceExtension, Services, EPP_XMLNS};

// Request

#[derive(Debug, PartialEq, ToXml)]
#[xml(rename = "hello", ns(EPP_XMLNS))]
pub(crate) struct Hello;

// Response

/// Type for data within the <svcMenu> section of an EPP greeting
#[derive(Debug, Eq, PartialEq)]
pub struct ServiceMenu {
    pub options: Options<'static>,
    pub services: Services<'static>,
}

/// Simplified service menu type for deserialization to `ServiceMenu` type from EPP greeting XML
#[derive(Debug, FromXml, PartialEq)]
#[xml(ns(EPP_XMLNS), rename = "svcMenu")]
struct FlattenedServiceMenu {
    pub version: String,
    pub lang: String,
    #[xml(rename = "objURI")]
    pub obj_uris: Vec<String>,
    #[xml(rename = "svcExtension")]
    pub svc_ext: Option<ServiceExtension<'static>>,
}

impl<'xml> FromXml<'xml> for ServiceMenu {
    fn matches(id: instant_xml::Id<'_>, field: Option<instant_xml::Id<'_>>) -> bool {
        FlattenedServiceMenu::matches(id, field)
    }

    /// Deserializes the <svcMenu> data to the `ServiceMenu` type
    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        field: &'static str,
        deserializer: &mut Deserializer<'cx, 'xml>,
    ) -> Result<(), instant_xml::Error> {
        dbg!(&into);

        let mut value = None;
        FlattenedServiceMenu::deserialize(&mut value, field, deserializer)?;
        let flattened = match value {
            Some(value) => value,
            None => return Ok(()),
        };

        *into = Some(ServiceMenu {
            options: Options {
                version: flattened.version.into(),
                lang: flattened.lang.into(),
            },
            services: Services {
                obj_uris: flattened.obj_uris.into_iter().map(|s| s.into()).collect(),
                svc_ext: flattened.svc_ext,
            },
        });

        Ok(())
    }

    type Accumulator = Option<Self>;
    const KIND: instant_xml::Kind = FlattenedServiceMenu::KIND;
}

/// Type corresponding to <all> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "all", ns(EPP_XMLNS))]
pub struct All;

/// Type corresponding to <none> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "noAccess", ns(EPP_XMLNS))]
pub struct NoAccess;

/// Type corresponding to <null> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "null", ns(EPP_XMLNS))]
pub struct Null;

/// Type corresponding to <personal> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "personal", ns(EPP_XMLNS))]
pub struct Personal;

/// Type corresponding to <personalAndOther> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "personalAndOther", ns(EPP_XMLNS))]
pub struct PersonalAndOther;

/// Type corresponding to <other> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "other", ns(EPP_XMLNS))]
pub struct Other;

/// Type corresponding to possible <retention> type values
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(forward)]
pub enum AccessType {
    /// Data for the <all> tag
    All(All),
    /// Data for the <none> tag
    NoAccess(NoAccess),
    /// Data for the <null> tag
    Null(Null),
    /// Data for the <personal> tag
    Personal(Personal),
    /// Data for the <personalAndOther> tag
    PersonalAndOther(PersonalAndOther),
    /// Data for the <other> tag
    Other(Other),
}

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "access", ns(EPP_XMLNS))]
pub struct Access {
    inner: AccessType,
}

/// Type corresponding to possible <purpose> type values
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(forward)]
pub enum PurposeType {
    /// Data for the <admin> tag
    Admin(Admin),
    /// Data for the <contact> tag
    Contact(Contact),
    /// Data for the <prov> tag
    Prov(Prov),
    /// Data for the <other> tag
    OtherPurpose(OtherPurpose),
}

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "admin", ns(EPP_XMLNS))]
pub struct Admin;

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "contact", ns(EPP_XMLNS))]
pub struct Contact;

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "prov", ns(EPP_XMLNS))]
pub struct Prov;

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "otherPurpose", ns(EPP_XMLNS))]
pub struct OtherPurpose;

/// Type corresponding to <purpose> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "purpose", ns(EPP_XMLNS))]
pub struct Purpose {
    pub purpose: Vec<PurposeType>,
}

/// Type corresponding to possible <purpose> type values
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(forward)]
pub enum RecipientType {
    /// Data for the <other> tag
    Other(Other),
    /// Data for the <ours> tag
    Ours(Ours),
    /// Data for the <public> tag
    Public(Public),
    /// Data for the <same> tag
    Same(Same),
    /// Data for the <unrelated> tag
    Unrelated(Unrelated),
}

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "ours", ns(EPP_XMLNS))]
pub struct Ours;

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "public", ns(EPP_XMLNS))]
pub struct Public;

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "unrelated", ns(EPP_XMLNS))]
pub struct Unrelated;

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "same", ns(EPP_XMLNS))]
pub struct Same;

/// Type corresponding to <recipeint> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "recipient", ns(EPP_XMLNS))]
pub struct Recipient {
    pub recipient: Vec<RecipientType>,
}

/// Type corresponding to <business> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "business", ns(EPP_XMLNS))]
pub struct Business;

/// Type corresponding to <indefinite> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "indefinite", ns(EPP_XMLNS))]
pub struct Indefinite;

/// Type corresponding to <legal> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "legal", ns(EPP_XMLNS))]
pub struct Legal;

/// Type corresponding to <none> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "none", ns(EPP_XMLNS))]
pub struct No;

/// Type corresponding to <stated> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "stated", ns(EPP_XMLNS))]
pub struct Stated;

/// Type corresponding to possible <retention> type values
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(forward, rename = "retention", ns(EPP_XMLNS))]
pub enum RetentionType {
    /// Data for the <business> tag
    Business(Business),
    /// Data for the <indefinite> tag
    Indefinite(Indefinite),
    /// Data for the <legal> tag
    Legal(Legal),
    /// Data for the <none> tag
    None(No),
    /// Data for the <stated> tag
    Stated(Stated),
}

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "retention", ns(EPP_XMLNS))]
pub struct Retention {
    inner: RetentionType,
}

/// Type corresponding to <statement> in the EPP greeting XML (pending more compliant implementation)
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "statement", ns(EPP_XMLNS))]
pub struct Statement {
    /// Data for the <purpose> tag
    pub purpose: Purpose,
    /// Data for the <recipient> tag
    pub recipient: Recipient,
    /// Data for the <retention> tag
    pub retention: Retention,
}

/// Type corresponding to <absolute> value in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "absolute", ns(EPP_XMLNS))]
pub struct Absolute(String);

/// Type corresponding to <relative> value in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "relative", ns(EPP_XMLNS))]
pub struct Relative(String);

/// Type corresponding to possible <expiry> type values
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(forward)]
pub enum ExpiryType {
    /// Data for the <absolute> tag
    Absolute(Absolute),
    /// Data for the <relative> tag
    Relative(Relative),
}

/// Type corresponding to possible <expiry> type values
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "expiry", ns(EPP_XMLNS))]
pub struct Expiry {
    inner: ExpiryType,
}

/// Type corresponding to <dcp> in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "dcp", ns(EPP_XMLNS))]
pub struct Dcp {
    /// Data for the <access> tag
    pub access: Access,
    /// Data for the <statement> tags
    pub statement: Vec<Statement>,
    /// Data for the <expiry> tag
    pub expiry: Option<Expiry>,
}

/// Type corresponding to the <greeting> tag in the EPP greeting XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(ns(EPP_XMLNS), rename = "greeting", rename_all = "lowercase")]
pub struct Greeting {
    /// The service ID
    #[xml(rename = "svID")]
    pub service_id: String,
    /// The date from the EPP server
    #[xml(rename = "svDate")]
    pub service_date: DateTime<Utc>,
    /// Data under the <svcMenu> element
    pub svc_menu: ServiceMenu,
    /// Data under the <dcp> element
    pub dcp: Dcp,
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::{ExpiryType, Greeting, Hello, Relative};
    use crate::tests::get_xml;
    use crate::xml;

    #[test]
    fn hello() {
        let xml = get_xml("request/hello.xml").unwrap();
        let serialized = xml::serialize(Hello).unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn greeting() {
        let xml = get_xml("response/greeting.xml").unwrap();
        let object = xml::deserialize::<Greeting>(xml.as_str()).unwrap();

        assert_eq!(object.service_id, "ISPAPI EPP Server");
        assert_eq!(
            object.service_date,
            Utc.with_ymd_and_hms(2021, 7, 25, 14, 51, 17).unwrap()
        );
        assert_eq!(object.svc_menu.options.version, "1.0");
        assert_eq!(object.svc_menu.options.lang, "en");
        assert_eq!(object.svc_menu.services.obj_uris.len(), 4);
        assert_eq!(
            object
                .svc_menu
                .services
                .svc_ext
                .unwrap()
                .ext_uris
                .unwrap()
                .len(),
            5
        );
        assert_eq!(object.dcp.statement.len(), 2);
        assert_eq!(
            object.dcp.expiry.unwrap().inner,
            ExpiryType::Relative(Relative("P1M".into()))
        );
    }
}
