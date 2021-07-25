//! Types for EPP contact create request

use epp_client_macros::*;

use crate::epp::object::data::{AuthInfo, ContactStatus, Phone, PostalInfo};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::response::contact::info::EppContactInfoResponse;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use crate::error;
use serde::{Deserialize, Serialize};

/// Type that represents the <epp> request for contact <update> command
pub type EppContactUpdate = EppObject<Command<ContactUpdate>>;

/// Type for elements under the <chg> tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactChangeInfo {
    #[serde(rename = "postalInfo")]
    postal_info: Option<PostalInfo>,
    voice: Option<Phone>,
    fax: Option<Phone>,
    email: Option<StringValue>,
    #[serde(rename = "authInfo")]
    auth_info: Option<AuthInfo>,
}

/// Type for list of elements of the <status> tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusList {
    status: Vec<ContactStatus>,
}

/// Type for elements under the contact <update> tag
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactUpdateData {
    xmlns: String,
    id: StringValue,
    #[serde(rename = "add")]
    add_statuses: Option<StatusList>,
    #[serde(rename = "rem")]
    remove_statuses: Option<StatusList>,
    #[serde(rename = "chg")]
    change_info: Option<ContactChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML <update> command for contacts
pub struct ContactUpdate {
    /// The data under the <update> tag for the contact update
    #[serde(rename = "update")]
    contact: ContactUpdateData,
}

impl EppContactUpdate {
    /// Creates a new EppObject for contact update corresponding to the <epp> tag in EPP XML
    pub fn new(id: &str, client_tr_id: &str) -> EppContactUpdate {
        EppObject::build(Command::<ContactUpdate> {
            command: ContactUpdate {
                contact: ContactUpdateData {
                    xmlns: EPP_CONTACT_XMLNS.to_string(),
                    id: id.to_string_value(),
                    add_statuses: None,
                    remove_statuses: None,
                    change_info: None,
                },
            },
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the data for the <chg> tag for the contact update request
    pub fn set_info(
        &mut self,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
    ) {
        self.data.command.contact.change_info = Some(ContactChangeInfo {
            email: Some(email.to_string_value()),
            postal_info: Some(postal_info),
            voice: Some(voice),
            auth_info: Some(AuthInfo::new(auth_password)),
            fax: None,
        });
    }

    /// Sets the data for the <fax> tag under <chg> for the contact update request
    pub fn set_fax(&mut self, fax: Phone) {
        match &mut self.data.command.contact.change_info {
            Some(ref mut info) => info.fax = Some(fax),
            _ => (),
        }
    }

    /// Sets the data for the <add> tag for the contact update request
    pub fn add_statuses(&mut self, statuses: Vec<ContactStatus>) {
        self.data.command.contact.add_statuses = Some(StatusList { status: statuses });
    }

    /// Sets the data for the <rem> tag for the contact update request
    pub fn remove_statuses(&mut self, statuses: Vec<ContactStatus>) {
        self.data.command.contact.remove_statuses = Some(StatusList { status: statuses });
    }

    /// Loads data into the <chg> tag from an existing EppContactInfoResponse object
    pub fn load_from_epp_contact_info(
        &mut self,
        contact_info: EppContactInfoResponse,
    ) -> Result<(), error::Error> {
        match contact_info.data.res_data {
            Some(res_data) => {
                self.data.command.contact.change_info = Some(ContactChangeInfo {
                    email: Some(res_data.info_data.email.clone()),
                    postal_info: Some(res_data.info_data.postal_info.clone()),
                    voice: Some(res_data.info_data.voice.clone()),
                    fax: res_data.info_data.fax.clone(),
                    auth_info: None,
                });
                Ok(())
            }
            None => Err(error::Error::Other(
                "No res_data in EppContactInfoResponse object".to_string(),
            )),
        }
    }
}
