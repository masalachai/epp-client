//! Types for EPP responses

use std::fmt::{self, Debug};

use chrono::{DateTime, Utc};
use instant_xml::{FromXml, Kind};

use crate::common::EPP_XMLNS;

/// Type corresponding to the <undef> tag an EPP response XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "undef", ns(EPP_XMLNS))]
pub struct Undef;

/// Type corresponding to the <value> tag under <extValue> in an EPP response XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "value", ns(EPP_XMLNS))]
pub struct ResultValue {
    /// The <undef> element
    pub undef: Undef,
}

/// Type corresponding to the <extValue> tag in an EPP response XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "extValue", ns(EPP_XMLNS))]
pub struct ExtValue {
    /// Data under the <value> tag
    pub value: ResultValue,
    /// Data under the <reason> tag
    pub reason: String,
}

/// Type corresponding to the <result> tag in an EPP response XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "result", ns(EPP_XMLNS))]
pub struct EppResult {
    /// The result code
    #[xml(attribute)]
    pub code: ResultCode,
    /// The result message
    #[xml(rename = "msg")]
    pub message: String,
    /// Data under the <extValue> tag
    pub ext_value: Option<ExtValue>,
}

/// Response codes as enumerated in section 3 of RFC 5730
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl<'xml> FromXml<'xml> for ResultCode {
    fn matches(id: instant_xml::Id<'_>, field: Option<instant_xml::Id<'_>>) -> bool {
        match field {
            Some(field) => id == field,
            None => false,
        }
    }

    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        field: &'static str,
        deserializer: &mut instant_xml::Deserializer<'cx, 'xml>,
    ) -> Result<(), instant_xml::Error> {
        let mut value = None;
        u16::deserialize(&mut value, field, deserializer)?;
        if let Some(value) = value {
            *into = match ResultCode::from_u16(value) {
                Some(value) => Some(value),
                None => {
                    return Err(instant_xml::Error::UnexpectedValue(format!(
                        "unexpected result code '{value}'"
                    )))
                }
            };
        }

        Ok(())
    }

    type Accumulator = Option<Self>;
    const KIND: instant_xml::Kind = Kind::Scalar;
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
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "trID", ns(EPP_XMLNS))]
pub struct ResponseTRID {
    /// The client TRID
    #[xml(rename = "clTRID")]
    pub client_tr_id: Option<String>,
    /// The server TRID
    #[xml(rename = "svTRID")]
    pub server_tr_id: String,
}

/// Type corresponding to the <msgQ> tag in an EPP response XML
#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "msgQ", ns(EPP_XMLNS))]
pub struct MessageQueue {
    /// The message count
    #[xml(attribute)]
    pub count: u32,
    /// The message ID
    #[xml(attribute)]
    pub id: String,
    /// The message date
    #[xml(rename = "qDate")]
    pub date: Option<DateTime<Utc>>,
    /// The message text
    #[xml(rename = "msg")]
    pub message: Option<Message>,
}

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "msg", ns(EPP_XMLNS))]
pub struct Message {
    #[xml(attribute)]
    pub lang: Option<String>,
    #[xml(direct)]
    pub text: String,
}

#[derive(Debug, FromXml, PartialEq)]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// containing an &lt;extension&gt; tag
#[xml(rename = "response", ns(EPP_XMLNS))]
pub struct Response<D, E> {
    /// Data under the <result> tag
    pub result: EppResult,
    /// Data under the <msgQ> tag
    #[xml(rename = "msgQ")]
    pub message_queue: Option<MessageQueue>,
    /// Data under the &lt;resData&gt; tag
    pub res_data: Option<ResponseData<D>>,
    /// Data under the &lt;extension&gt; tag
    pub extension: Option<Extension<E>>,
    /// Data under the <trID> tag
    pub tr_ids: ResponseTRID,
}

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "resData", ns(EPP_XMLNS))]
pub struct ResponseData<D> {
    data: D,
}

impl<D> ResponseData<D> {
    pub fn into_inner(self) -> D {
        self.data
    }
}

#[derive(Debug, FromXml, PartialEq)]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// without <msgQ> or &lt;resData&gt; sections. Generally used for error handling
#[xml(rename = "response", ns(EPP_XMLNS))]
pub struct ResponseStatus {
    /// Data under the <result> tag
    pub result: EppResult,
    #[xml(rename = "trID")]
    /// Data under the <trID> tag
    pub tr_ids: ResponseTRID,
}

impl<T, E> Response<T, E> {
    /// Returns the data under the corresponding &lt;resData&gt; from the EPP XML
    pub fn res_data(&self) -> Option<&T> {
        match &self.res_data {
            Some(res_data) => Some(&res_data.data),
            None => None,
        }
    }

    pub fn extension(&self) -> Option<&E> {
        match &self.extension {
            Some(extension) => Some(&extension.data),
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

#[derive(Debug, Eq, FromXml, PartialEq)]
#[xml(rename = "extension", ns(EPP_XMLNS))]
pub struct Extension<E> {
    pub data: E,
}

#[cfg(test)]
mod tests {
    use super::{ResponseStatus, ResultCode};
    use crate::tests::{get_xml, CLTRID, SVTRID};
    use crate::xml;

    #[test]
    fn error() {
        let xml = get_xml("response/error.xml").unwrap();
        let object = xml::deserialize::<ResponseStatus>(xml.as_str()).unwrap();

        assert_eq!(object.result.code, ResultCode::ObjectDoesNotExist);
        assert_eq!(object.result.message, "Object does not exist");
        assert_eq!(
            object.result.ext_value.unwrap().reason,
            "545 Object not found"
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID);
        assert_eq!(object.tr_ids.server_tr_id, SVTRID);
    }
}
