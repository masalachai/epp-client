//! Types for EPP responses

pub mod contact;
pub mod domain;
pub mod host;
pub mod message;

use epp_client_macros::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Debug;

use crate::epp::object::{
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

/// Type corresponding to <all> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct All;

/// Type corresponding to <access> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Access {
    /// Data for the <all> tag
    pub all: All,
}

/// Type corresponding to <admin> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Admin;

/// Type corresponding to <prov> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Prov;

/// Type corresponding to <purpose> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Purpose {
    /// Data for the <admin> tag
    pub admin: Admin,
    /// Data for the <prov> tag
    pub prov: Prov,
}

/// Type corresponding to <ours> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ours;

/// Type corresponding to <public> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Public;

/// Type corresponding to <recipeint> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipient {
    /// Data for the <ours> tag
    pub ours: Ours,
    /// Data for the <public> tag
    pub public: Public,
}

/// Type corresponding to <stated> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Stated;

/// Type corresponding to <retention> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Retention {
    /// Data for the <stated> tag
    pub stated: Stated,
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

/// Type corresponding to <dcp> in the EPP greeting XML (pending more compliant implementation)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dcp {
    /// Data for the <access> tag
    pub access: Access,
    /// Data for the <statement> tag
    pub statement: Statement,
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
