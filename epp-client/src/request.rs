//! Types for EPP requests

use serde::{de::DeserializeOwned, ser::SerializeStruct, ser::Serializer, Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::time::SystemTime;

use crate::{
    common::NoExtension,
    common::{ElementName, EppObject, Extension, StringValue},
    response::{ResponseStatus, ResponseWithExtension},
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
        EppXml::serialize(&EppObject::build(CommandWithExtension {
            command,
            extension,
            client_tr_id: client_tr_id.into(),
        }))
    }

    fn deserialize_response(
        epp_xml: &str,
    ) -> Result<ResponseWithExtension<Self::Output, E::Response>, crate::error::Error> {
        let rsp =
            <EppObject<ResponseWithExtension<Self::Output, E::Response>> as EppXml>::deserialize(
                epp_xml,
            )?;
        match rsp.data.result.code {
            0..=2000 => Ok(rsp.data),
            _ => Err(crate::error::Error::EppCommandError(EppObject::build(
                ResponseStatus {
                    result: rsp.data.result,
                    tr_ids: rsp.data.tr_ids,
                },
            ))),
        }
    }
}

pub trait EppExtension: ElementName + DeserializeOwned + Serialize + Sized + Debug {
    type Response: ElementName + DeserializeOwned + Serialize + Debug;
}

/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// without an &lt;extension&gt; tag
pub type Command<T> = CommandWithExtension<T, NoExtension>;

#[derive(Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "command")]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
pub struct CommandWithExtension<T: ElementName, E: ElementName> {
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub command: T,
    /// The client TRID
    pub extension: Option<Extension<E>>,
    #[serde(rename = "clTRID")]
    pub client_tr_id: StringValue,
}

impl<T: ElementName + Serialize, E: ElementName + Serialize> Serialize
    for CommandWithExtension<T, E>
{
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

impl<T: ElementName> Command<T> {
    /// Creates a new &lt;command&gt; tag for an EPP document
    pub fn new(command: T, client_tr_id: &str) -> Command<T> {
        Command {
            command,
            extension: None,
            client_tr_id: client_tr_id.into(),
        }
    }
}

impl<T: ElementName, E: ElementName> CommandWithExtension<T, E> {
    /// Creates a new &lt;command&gt; tag for an EPP document with a containing &lt;extension&gt; tag
    pub fn build(command: T, ext: E, client_tr_id: &str) -> CommandWithExtension<T, E> {
        CommandWithExtension {
            command,
            extension: Some(Extension { data: ext }),
            client_tr_id: client_tr_id.into(),
        }
    }
}

/// Basic client TRID generation function. Mainly used for testing. Users of the library should use their own clTRID generation function.
pub fn generate_client_tr_id(username: &str) -> Result<String, Box<dyn Error>> {
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    Ok(format!("{}:{}", username, timestamp.as_secs()))
}
