//! Types for EPP requests

use std::fmt::Debug;

use instant_xml::{FromXmlOwned, ToXml};

use crate::common::EPP_XMLNS;

pub const EPP_VERSION: &str = "1.0";
pub const EPP_LANG: &str = "en";

/// Trait to set correct value for xml tags when tags are being generated from generic types
pub trait Transaction<Ext: Extension>: Command + Sized {}

pub trait Command: ToXml + Debug {
    type Response: FromXmlOwned + Debug;
    const COMMAND: &'static str;
}

pub trait Extension: ToXml + Debug {
    type Response: FromXmlOwned + Debug;
}

#[derive(Debug, PartialEq)]
/// Type corresponding to the &lt;command&gt; tag in an EPP XML request
/// with an &lt;extension&gt; tag
pub(crate) struct CommandWrapper<'a, D, E> {
    pub command: &'static str,
    /// The instance that will be used to populate the &lt;command&gt; tag
    pub data: &'a D,
    /// The client TRID
    pub extension: Option<&'a E>,
    pub client_tr_id: String,
}

impl<'a, E: Extension, D: Transaction<E>> CommandWrapper<'a, D, E> {
    pub(crate) fn new(data: &'a D, extension: Option<&'a E>, client_tr_id: &'a str) -> Self {
        Self {
            command: D::COMMAND,
            data,
            extension,
            client_tr_id: client_tr_id.into(),
        }
    }
}

impl<'a, D: ToXml, E: ToXml> ToXml for CommandWrapper<'a, D, E> {
    fn serialize<W: std::fmt::Write + ?Sized>(
        &self,
        _: Option<instant_xml::Id<'_>>,
        serializer: &mut instant_xml::Serializer<W>,
    ) -> Result<(), instant_xml::Error> {
        let prefix = serializer.write_start("command", EPP_XMLNS)?;
        serializer.end_start()?;
        self.data.serialize(None, serializer)?;
        if let Some(extension) = self.extension {
            Ext { inner: extension }.serialize(None, serializer)?;
        }

        let id_prefix = serializer.write_start("clTRID", EPP_XMLNS)?;
        serializer.end_start()?;
        serializer.write_str(&self.client_tr_id)?;
        serializer.write_close(id_prefix, "clTRID")?;

        serializer.write_close(prefix, "command")?;
        Ok(())
    }
}

#[derive(Debug, ToXml)]
#[xml(rename = "extension", ns(EPP_XMLNS))]
struct Ext<E> {
    inner: E,
}
