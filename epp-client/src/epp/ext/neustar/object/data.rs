//! Common data types for EPP Requests and Responses for Neustar

use epp_client_macros::*;

use crate::epp::object::{ElementName, StringValue};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "extension")]
/// Type that represents the &lt;extension&gt; tag for Neustar extensions
pub struct Extension {
    /// The XML namespace for the Neustar &lt;extension&gt; tag
    pub xmlns: String,
    /// XML schema location for the Neulevel extension
    #[serde(rename = "xsi:schemaLocation")]
    pub schema_location: String,
    /// The string under the &lt;unspec&gt; tag
    pub unspec: StringValue,
}

/// Type to hold key/value pairs that are set in the &lt;unspec&gt; tag
pub struct ContactExtension {
    pub ext_contact: Option<bool>,
    pub app_purpose: Option<String>,
    pub nexus_category: Option<String>,
}

impl fmt::Display for ContactExtension {
    /// Converts `ContactExtension` instance into string format that can be used inside
    /// the &lt;unspec&gt; tag
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ext_str = "".to_string();

        if let Some(ext_contact) = self.ext_contact {
            ext_str = {
                match ext_contact {
                    true => format!("extContact=Y"),
                    false => format!("extContact=N"),
                }
            }
        }

        if let Some(ref app_purpose) = &self.app_purpose {
            ext_str = format!("{} appPurpose={}", ext_str, app_purpose);
        }

        if let Some(ref nexus_category) = &self.nexus_category {
            ext_str = format!("{} nexusCategory={}", ext_str, nexus_category);
        }

        write!(f, "{}", ext_str.trim())
    }
}
