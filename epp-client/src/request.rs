//! Types for EPP requests

use serde::{de::DeserializeOwned, ser::SerializeStruct, ser::Serializer, Deserialize, Serialize};
use std::fmt::Debug;

use crate::{
    common::{ElementName, EppObject, Extension, StringValue},
    response::{Response, ResponseStatus},
    xml::EppXml,
};
use epp_client_macros::ElementName;

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait EppRequest<E: EppExtension>: Sized + Debug {
    type Input: ElementName + DeserializeOwned + Serialize + Sized + Debug;
    type Output: DeserializeOwned + Serialize + Debug;

    fn into_parts(self) -> (Self::Input, Option<E>);

    fn serialize_request(self, client_tr_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (command, extension) = self.into_parts();
        let extension = extension.map(|data| Extension { data });
        EppXml::serialize(&EppObject::build(Command {
            command,
            extension,
            client_tr_id: client_tr_id.into(),
        }))
    }

    fn deserialize_response(
        epp_xml: &str,
    ) -> Result<Response<Self::Output, E::Response>, crate::error::Error> {
        let rsp = <EppObject<Response<Self::Output, E::Response>> as EppXml>::deserialize(epp_xml)?;
        match rsp.data.result.code {
            0..=2000 => Ok(rsp.data),
            _ => Err(crate::error::Error::EppCommandError(ResponseStatus {
                result: rsp.data.result,
                tr_ids: rsp.data.tr_ids,
            })),
        }
    }
}

pub trait EppExtension: ElementName + DeserializeOwned + Serialize + Sized + Debug {
    type Response: ElementName + DeserializeOwned + Serialize + Debug;
}

#[derive(Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "command")]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
pub struct Command<T: ElementName, E: ElementName> {
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub command: T,
    /// The client TRID
    pub extension: Option<Extension<E>>,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<T: ElementName + Serialize, E: ElementName + Serialize> Serialize for Command<T, E> {
    /// Serializes the generic type T to the proper XML tag (set by the `#[element_name(name = <tagname>)]` attribute) for the request
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("command", 3)?;
        state.serialize_field(T::ELEMENT, &self.command)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}

impl<T: ElementName, E: ElementName> Command<T, E> {
    /// Creates a new &lt;command&gt; tag for an EPP document with a containing &lt;extension&gt; tag
    pub fn build(command: T, ext: E, client_tr_id: &str) -> Command<T, E> {
        Command {
            command,
            extension: Some(Extension { data: ext }),
            client_tr_id: client_tr_id.into(),
        }
    }
}
