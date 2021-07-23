use epp_client_macros::*;

use crate::epp::command::Command;
use crate::epp::object::data::AuthInfo;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

pub type EppContactInfo = EppObject<Command<ContactInfo>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactInfoData {
    xmlns: String,
    id: StringValue,
    #[serde(rename = "authInfo")]
    auth_info: AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "info")]
pub struct ContactInfo {
    #[serde(rename = "info")]
    info: ContactInfoData,
}

impl EppContactInfo {
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
