//! Types for EPP contact create request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{
    ContactAuthInfo, ContactStatus, ElementName, NoExtension, Phone, PostalInfo, StringValue,
};
use crate::request::{EppExtension, Transaction};
use crate::response::ResponseStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ContactUpdate<E> {
    request: ContactUpdateRequest,
    extension: Option<E>,
}

impl<E: EppExtension> Transaction<E> for ContactUpdate<E> {
    type Input = ContactUpdateRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> ContactUpdate<E> {
    pub fn new(id: &str) -> ContactUpdate<NoExtension> {
        ContactUpdate {
            request: ContactUpdateRequest {
                contact: ContactUpdateRequestData {
                    xmlns: XMLNS.to_string(),
                    id: id.into(),
                    add_statuses: None,
                    remove_statuses: None,
                    change_info: None,
                },
            },
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> ContactUpdate<F> {
        ContactUpdate {
            request: self.request,
            extension: Some(extension),
        }
    }

    /// Sets the data for the &lt;chg&gt; tag for the contact update request
    pub fn set_info(
        &mut self,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
    ) {
        self.request.contact.change_info = Some(ContactChangeInfo {
            email: Some(email.into()),
            postal_info: Some(postal_info),
            voice: Some(voice),
            auth_info: Some(ContactAuthInfo::new(auth_password)),
            fax: None,
        });
    }

    /// Sets the data for the &lt;fax&gt; tag under &lt;chg&gt; for the contact update request
    pub fn set_fax(&mut self, fax: Phone) {
        if let Some(info) = &mut self.request.contact.change_info {
            info.fax = Some(fax)
        }
    }

    /// Sets the data for the &lt;add&gt; tag for the contact update request
    pub fn add(&mut self, statuses: Vec<ContactStatus>) {
        self.request.contact.add_statuses = Some(StatusList { status: statuses });
    }

    /// Sets the data for the &lt;rem&gt; tag for the contact update request
    pub fn remove(&mut self, statuses: Vec<ContactStatus>) {
        self.request.contact.remove_statuses = Some(StatusList { status: statuses });
    }
}

/// Type for elements under the &lt;chg&gt; tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactChangeInfo {
    #[serde(rename = "contact:postalInfo", alias = "postalInfo")]
    postal_info: Option<PostalInfo>,
    #[serde(rename = "contact:voice", alias = "voice")]
    voice: Option<Phone>,
    #[serde(rename = "contact:fax", alias = "fax")]
    fax: Option<Phone>,
    #[serde(rename = "contact:email", alias = "email")]
    email: Option<StringValue>,
    #[serde(rename = "contact:authInfo", alias = "authInfo")]
    auth_info: Option<ContactAuthInfo>,
}

/// Type for list of elements of the &lt;status&gt; tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusList {
    #[serde(rename = "contact:status", alias = "status")]
    status: Vec<ContactStatus>,
}

/// Type for elements under the contact &lt;update&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactUpdateRequestData {
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
    #[serde(rename = "contact:add", alias = "add")]
    add_statuses: Option<StatusList>,
    #[serde(rename = "contact:rem", alias = "rem")]
    remove_statuses: Option<StatusList>,
    #[serde(rename = "contact:chg", alias = "chg")]
    change_info: Option<ContactChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for contacts
pub struct ContactUpdateRequest {
    /// The data under the &lt;update&gt; tag for the contact update
    #[serde(rename = "contact:update", alias = "update")]
    contact: ContactUpdateRequestData,
}
