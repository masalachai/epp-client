//! Types for EPP responses

use std::fmt::{self, Debug};

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};

use crate::common::StringValue;
use crate::xml::EppXml;

/// Type corresponding to the <undef> tag an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct Undef;

/// Type corresponding to the <value> tag under <extValue> in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct ResultValue {
    /// The XML namespace for the <value> tag
    #[serde(rename = "xmlns:epp")]
    xmlns: String,
    /// The <undef> element
    pub undef: Undef,
}

/// Type corresponding to the <extValue> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct ExtValue {
    /// Data under the <value> tag
    pub value: ResultValue,
    /// Data under the <reason> tag
    pub reason: StringValue<'static>,
}

/// Type corresponding to the <result> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct EppResult {
    /// The result code
    pub code: ResultCode,
    /// The result message
    #[serde(rename = "msg")]
    pub message: StringValue<'static>,
    /// Data under the <extValue> tag
    #[serde(rename = "extValue")]
    pub ext_value: Option<ExtValue>,
}

/// Response codes as enumerated in section 3 of RFC 5730
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResultCode {
    CommandCompletedSuccessfully = 1000,
    CommandCompletedSuccessfullyActionPending = 1001,
    CommandCompletedSuccessfullyNoMessages = 1300,
    CommandCompletedSuccessfullyAckToDequeue = 1301,
    CommandCompletedSuccessfullyEndingSession = 1500,
    UnknownCommand = 2000,
    CommandSyntaxError = 2001,
    CommandUseError = 2002,
    RequiredParameterMissing = 2003,
    ParameterValueRangeError = 2004,
    ParameterValueSyntaxError = 2005,
    UnimplementedProtocolVersion = 2100,
    UnimplementedCommand = 2101,
    UnimplementedOption = 2102,
    UnimplementedExtension = 2103,
    BillingFailure = 2104,
    ObjectIsNotEligibleForRenewal = 2105,
    ObjectIsNotEligibleForTransfer = 2106,
    AuthenticationError = 2200,
    AuthorizationError = 2201,
    InvalidAuthorizationInformation = 2202,
    ObjectPendingTransfer = 2300,
    ObjectNotPendingTransfer = 2301,
    ObjectExists = 2302,
    ObjectDoesNotExist = 2303,
    ObjectStatusProhibitsOperation = 2304,
    ObjectAssociationProhibitsOperation = 2305,
    ParameterValuePolicyError = 2306,
    UnimplementedObjectService = 2307,
    DataManagementPolicyViolation = 2308,
    CommandFailed = 2400,
    CommandFailedServerClosingConnection = 2500,
    AuthenticationErrorServerClosingConnection = 2501,
    SessionLimitExceededServerClosingConnection = 2502,
}

impl ResultCode {
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            1000 => Some(ResultCode::CommandCompletedSuccessfully),
            1001 => Some(ResultCode::CommandCompletedSuccessfullyActionPending),
            1300 => Some(ResultCode::CommandCompletedSuccessfullyNoMessages),
            1301 => Some(ResultCode::CommandCompletedSuccessfullyAckToDequeue),
            1500 => Some(ResultCode::CommandCompletedSuccessfullyEndingSession),
            2000 => Some(ResultCode::UnknownCommand),
            2001 => Some(ResultCode::CommandSyntaxError),
            2002 => Some(ResultCode::CommandUseError),
            2003 => Some(ResultCode::RequiredParameterMissing),
            2004 => Some(ResultCode::ParameterValueRangeError),
            2005 => Some(ResultCode::ParameterValueSyntaxError),
            2100 => Some(ResultCode::UnimplementedProtocolVersion),
            2101 => Some(ResultCode::UnimplementedCommand),
            2102 => Some(ResultCode::UnimplementedOption),
            2103 => Some(ResultCode::UnimplementedExtension),
            2104 => Some(ResultCode::BillingFailure),
            2105 => Some(ResultCode::ObjectIsNotEligibleForRenewal),
            2106 => Some(ResultCode::ObjectIsNotEligibleForTransfer),
            2200 => Some(ResultCode::AuthenticationError),
            2201 => Some(ResultCode::AuthorizationError),
            2202 => Some(ResultCode::InvalidAuthorizationInformation),
            2300 => Some(ResultCode::ObjectPendingTransfer),
            2301 => Some(ResultCode::ObjectNotPendingTransfer),
            2302 => Some(ResultCode::ObjectExists),
            2303 => Some(ResultCode::ObjectDoesNotExist),
            2304 => Some(ResultCode::ObjectStatusProhibitsOperation),
            2305 => Some(ResultCode::ObjectAssociationProhibitsOperation),
            2306 => Some(ResultCode::ParameterValuePolicyError),
            2307 => Some(ResultCode::UnimplementedObjectService),
            2308 => Some(ResultCode::DataManagementPolicyViolation),
            2400 => Some(ResultCode::CommandFailed),
            2500 => Some(ResultCode::CommandFailedServerClosingConnection),
            2501 => Some(ResultCode::AuthenticationErrorServerClosingConnection),
            2502 => Some(ResultCode::SessionLimitExceededServerClosingConnection),
            _ => None,
        }
    }

    pub fn is_success(&self) -> bool {
        use ResultCode::*;
        matches!(
            self,
            CommandCompletedSuccessfully
                | CommandCompletedSuccessfullyActionPending
                | CommandCompletedSuccessfullyNoMessages
                | CommandCompletedSuccessfullyAckToDequeue
                | CommandCompletedSuccessfullyEndingSession
        )
    }
}

impl<'de> Deserialize<'de> for ResultCode {
    fn deserialize<D>(deserializer: D) -> Result<ResultCode, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(ResultCodeVisitor)
    }
}

struct ResultCodeVisitor;

impl<'de> serde::de::Visitor<'de> for ResultCodeVisitor {
    type Value = ResultCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid EPP result code")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use serde::de::Unexpected;
        ResultCode::from_u16(v).ok_or_else(|| {
            E::invalid_value(Unexpected::Unsigned(v as u64), &"unexpected result code")
        })
    }
}

/// Type corresponding to the <trID> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct ResponseTRID {
    /// The client TRID
    #[serde(rename = "clTRID")]
    pub client_tr_id: Option<StringValue<'static>>,
    /// The server TRID
    #[serde(rename = "svTRID")]
    pub server_tr_id: StringValue<'static>,
}

/// Type corresponding to the <msgQ> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct MessageQueue {
    /// The message count
    pub count: u32,
    /// The message ID
    pub id: String,
    /// The message date
    #[serde(rename = "qDate")]
    pub date: Option<DateTime<Utc>>,
    /// The message text
    #[serde(rename = "msg")]
    pub message: Option<StringValue<'static>>,
}

#[derive(Deserialize, Debug, PartialEq)]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// containing an &lt;extension&gt; tag
pub struct Response<D, E> {
    /// Data under the <result> tag
    pub result: EppResult,
    /// Data under the <msgQ> tag
    #[serde(rename = "msgQ")]
    pub message_queue: Option<MessageQueue>,
    #[serde(rename = "resData")]
    /// Data under the &lt;resData&gt; tag
    pub res_data: Option<D>,
    /// Data under the &lt;extension&gt; tag
    pub extension: Option<E>,
    /// Data under the <trID> tag
    #[serde(rename = "trID")]
    pub tr_ids: ResponseTRID,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct ResponseDocument<D, E> {
    #[serde(rename = "response")]
    pub data: Response<D, E>,
}

impl<D: DeserializeOwned, E: DeserializeOwned> EppXml for ResponseDocument<D, E> {}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "epp")]
pub struct ResultDocument {
    #[serde(rename = "response")]
    pub data: ResponseStatus,
}

impl EppXml for ResultDocument {}

#[derive(Deserialize, Debug, PartialEq)]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// without <msgQ> or &lt;resData&gt; sections. Generally used for error handling
pub struct ResponseStatus {
    /// Data under the <result> tag
    pub result: EppResult,
    #[serde(rename = "trID")]
    /// Data under the <trID> tag
    pub tr_ids: ResponseTRID,
}

impl<T, E> Response<T, E> {
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

#[cfg(test)]
mod tests {
    use super::{ResultCode, ResultDocument};
    use crate::tests::{get_xml, CLTRID, SVTRID};
    use crate::xml::EppXml;

    #[test]
    fn error() {
        let xml = get_xml("response/error.xml").unwrap();
        let object = ResultDocument::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, ResultCode::ObjectDoesNotExist);
        assert_eq!(object.data.result.message, "Object does not exist".into());
        assert_eq!(
            object.data.result.ext_value.unwrap().reason,
            "545 Object not found".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }
}
