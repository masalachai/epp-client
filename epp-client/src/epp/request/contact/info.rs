//! Types for EPP contact info request

use epp_client_macros::*;

use crate::epp::object::data::AuthInfo;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type for the <epp> request for contact <info> command
pub type EppContactInfo = EppObject<Command<ContactInfo>>;

/// Type for elements under the contact <info> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoData {
    /// XML namespace for contact commands
    xmlns: String,
    /// The contact id for the info command
    id: StringValue,
    /// The <authInfo> data
    #[serde(rename = "authInfo")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
/// Type for EPP XML <info> command for contacts
pub struct ContactInfo {
    /// Data for <info> command for contact
    #[serde(rename = "info")]
    info: ContactInfoData,
}

impl EppContactInfo {
    /// Creates a new EppObject for contact info corresponding to the <epp> tag in EPP XML
    pub fn new(id: &str, auth_password: &str, client_tr_id: &str) -> EppContactInfo {
        EppObject::build(Command::<ContactInfo> {
            command: ContactInfo {
                info: ContactInfoData {
                    xmlns: EPP_CONTACT_XMLNS.to_string(),
                    id: id.to_string_value(),
                    auth_info: AuthInfo::new(auth_password),
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }
}
