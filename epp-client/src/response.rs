//! Types for EPP responses

use epp_client_macros::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;

use crate::common::{
    ElementName, EmptyTag, EppObject, Extension, Options, ServiceExtension, Services, StringValue,
};

/// Type corresponding to the &lt;response&gt; tag in an EPP response without an &lt;extension&gt; section
pub type CommandResponse<T> = CommandResponseWithExtension<T, EmptyTag>;

/// The EPP Greeting that is received on a successful connection and in response to an EPP hello
pub type EppGreeting = EppObject<Greeting>;
/// A generic EPP Response to an EPP command with a result section, a status code and a message
pub type EppCommandResponse = EppObject<CommandResponseStatus>;
/// An alias of `EppCommandResponse` indicating an EPP Error
pub type EppCommandResponseError = EppCommandResponse;
/// An alias of `EppCommandResponse` received in response to a successful login request
pub type EppLoginResponse = EppCommandResponse;
/// An alias of `EppCommandResponse` received in response to a successful logout request
pub type EppLogoutResponse = EppCommandResponse;

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

/// Type corresponding to the <undef> tag an EPP response XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Undef;

/// Type corresponding to the <value> tag under <extValue> in an EPP response XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResultValue {
    /// The XML namespace for the <value> tag
    #[serde(rename = "xmlns:epp")]
    xmlns: String,
    /// The <undef> element
    pub undef: Undef,
}

/// Type corresponding to the <extValue> tag in an EPP response XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtValue {
    /// Data under the <value> tag
    pub value: ResultValue,
    /// Data under the <reason> tag
    pub reason: StringValue,
}

/// Type corresponding to the <result> tag in an EPP response XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EppResult {
    /// The result code
    pub code: u16,
    /// The result message
    #[serde(rename = "msg")]
    pub message: StringValue,
    /// Data under the <extValue> tag
    #[serde(rename = "extValue")]
    pub ext_value: Option<ExtValue>,
}

/// Type corresponding to the <trID> tag in an EPP response XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResponseTRID {
    /// The client TRID
    #[serde(rename = "clTRID")]
    pub client_tr_id: Option<StringValue>,
    /// The server TRID
    #[serde(rename = "svTRID")]
    pub server_tr_id: StringValue,
}

/// Type corresponding to the <msgQ> tag in an EPP response XML
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MessageQueue {
    /// The message count
    pub count: u32,
    /// The message ID
    pub id: String,
    /// The message date
    #[serde(rename = "qDate")]
    pub date: Option<StringValue>,
    /// The message text
    #[serde(rename = "msg")]
    pub message: Option<StringValue>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[serde(rename_all = "lowercase")]
#[element_name(name = "response")]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// containing an &lt;extension&gt; tag
pub struct CommandResponseWithExtension<T, E: ElementName> {
    /// Data under the <result> tag
    pub result: EppResult,
    /// Data under the <msgQ> tag
    #[serde(rename = "msgQ")]
    pub message_queue: Option<MessageQueue>,
    #[serde(rename = "resData")]
    /// Data under the &lt;resData&gt; tag
    pub res_data: Option<T>,
    /// Data under the &lt;extension&gt; tag
    pub extension: Option<Extension<E>>,
    /// Data under the <trID> tag
    #[serde(rename = "trID")]
    pub tr_ids: ResponseTRID,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "response")]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// without <msgQ> or &lt;resData&gt; sections. Generally used for error handling
pub struct CommandResponseStatus {
    /// Data under the <result> tag
    pub result: EppResult,
    #[serde(rename = "trID")]
    /// Data under the <trID> tag
    pub tr_ids: ResponseTRID,
}

impl<T, E: ElementName> CommandResponseWithExtension<T, E> {
    /// Returns the data under the corresponding &lt;resData&gt; from the EPP XML
    pub fn res_data(&self) -> Option<&T> {
        match &self.res_data {
            Some(res_data) => Some(res_data),
            None => None,
        }
    }
    /// Returns the data under the corresponding <msgQ> from the EPP XML
    pub fn message_queue(&self) -> Option<&MessageQueue> {
        match &self.message_queue {
            Some(queue) => Some(queue),
            None => None,
        }
    }
}
