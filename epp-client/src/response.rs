//! Types for EPP responses

use epp_client_macros::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::common::NoExtension;
use crate::common::{ElementName, EppObject, Extension, StringValue};

/// Type corresponding to the &lt;response&gt; tag in an EPP response without an &lt;extension&gt; section
pub type CommandResponse<T> = CommandResponseWithExtension<T, NoExtension>;

/// A generic EPP Response to an EPP command with a result section, a status code and a message
pub type EppCommandResponse = EppObject<CommandResponseStatus>;
/// An alias of `EppCommandResponse` indicating an EPP Error
pub type EppCommandResponseError = EppCommandResponse;

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
