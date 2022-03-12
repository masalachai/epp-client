//! Types for EPP requests

use serde::{de::DeserializeOwned, ser::SerializeStruct, ser::Serializer, Serialize};
use std::fmt::Debug;

use crate::{
    common::{StringValue, EPP_XMLNS},
    xml::EppXml,
};

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait Transaction<Ext: Extension>: Command + Sized {}

pub trait Command: Serialize + Debug {
    type Response: DeserializeOwned + Debug;
    const COMMAND: &'static str;
}

pub trait Extension: Serialize + Debug {
    type Response: DeserializeOwned + Debug;
}

#[derive(Debug, PartialEq)]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
struct CommandWrapper<'a, D, E> {
    pub command: &'static str,
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub data: &'a D,
    /// The client TRID
    pub extension: Option<&'a E>,
    pub client_tr_id: StringValue<'a>,
}

impl<'a, D: Serialize, E: Serialize> Serialize for CommandWrapper<'a, D, E> {
    /// Serializes the generic type T to the proper XML tag (set by the `#[element_name(name = <tagname>)]` attribute) for the request
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("command", 3)?;
        state.serialize_field(self.command, self.data)?;
        state.serialize_field("extension", &self.extension)?;
        state.serialize_field("clTRID", &self.client_tr_id)?;
        state.end()
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename = "epp")]
pub(crate) struct CommandDocument<'a, Cmd, Ext> {
    xmlns: &'static str,
    command: CommandWrapper<'a, Cmd, Ext>,
}

impl<'a, Cmd, Ext> CommandDocument<'a, Cmd, Ext> {
    pub(crate) fn new(data: &'a Cmd, extension: Option<&'a Ext>, client_tr_id: &'a str) -> Self
    where
        Cmd: Transaction<Ext>,
        Ext: Extension,
    {
        Self {
            xmlns: EPP_XMLNS,
            command: CommandWrapper {
                command: Cmd::COMMAND,
                data,
                extension,
                client_tr_id: client_tr_id.into(),
            },
        }
    }
}

impl<'a, D: Serialize, E: Serialize> EppXml for CommandDocument<'a, D, E> {}
