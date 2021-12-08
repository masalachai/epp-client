//! Types for EPP requests

use serde::{de::DeserializeOwned, ser::SerializeStruct, ser::Serializer, Deserialize, Serialize};
use std::fmt::Debug;

use crate::{
    common::{ElementName, StringValue, EPP_XMLNS},
    response::{Response, ResponseDocument, ResponseStatus},
    xml::EppXml,
};
use epp_client_macros::ElementName;

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait EppRequest<E: EppExtension>: Sized + Debug {
    type Input: ElementName + Serialize + Sized + Debug;
    type Output: DeserializeOwned + Debug;

    fn into_parts(self) -> (Self::Input, Option<E>);

    fn serialize_request(self, client_tr_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (command, extension) = self.into_parts();
        <CommandDocument<Self::Input, E> as EppXml>::serialize(&CommandDocument::new(Command {
            command: <Self::Input as ElementName>::ELEMENT,
            data: command,
            extension,
            client_tr_id: client_tr_id.into(),
        }))
    }

    fn deserialize_response(
        epp_xml: &str,
    ) -> Result<Response<Self::Output, E::Response>, crate::error::Error> {
        let rsp = <ResponseDocument<Self::Output, E::Response> as EppXml>::deserialize(epp_xml)?;
        match rsp.data.result.code {
            0..=2000 => Ok(rsp.data),
            _ => Err(crate::error::Error::EppCommandError(ResponseStatus {
                result: rsp.data.result,
                tr_ids: rsp.data.tr_ids,
            })),
        }
    }
}

pub trait EppExtension: Serialize + Sized + Debug {
    type Response: DeserializeOwned + Debug;
}

#[derive(Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "command")]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
pub struct Command<D, E> {
    pub command: &'static str,
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub data: D,
    /// The client TRID
    pub extension: Option<E>,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<D: Serialize, E: Serialize> Serialize for Command<D, E> {
    /// Serializes the generic type T to the proper XML tag (set by the `#[element_name(name = <tagname>)]` attribute) for the request
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("command", 3)?;
        state.serialize_field(self.command, &self.data)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "epp")]
pub struct CommandDocument<D, E> {
    xmlns: &'static str,
    command: Command<D, E>,
}

impl<D, E> CommandDocument<D, E> {
    pub fn new(command: Command<D, E>) -> Self {
        Self {
            xmlns: EPP_XMLNS,
            command,
        }
    }
}

impl<D: Serialize, E: Serialize> EppXml for CommandDocument<D, E> {}
