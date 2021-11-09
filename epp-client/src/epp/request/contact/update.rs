//! Types for EPP contact create request

use epp_client_macros::*;

use crate::epp::object::data::{AuthInfo, ContactStatus, Phone, PostalInfo};
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::response::contact::info::{ContactInfoResult, EppContactInfoResponse};
use crate::epp::xml::EPP_CONTACT_XMLNS;
use crate::error;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for contact &lt;update&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppContactUpdate, EppContactUpdateResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create an instance of EppClient, specifying the name of the registry as in
///     // the config file
///     let mut client = match EppClient::new("verisign").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an EppContactUpdate instance
///     let mut contact_update = EppContactUpdate::new(
///         "eppdev-contact-100",
///         generate_client_tr_id(&client).as_str()
///     );
///
///     let add_statuses = vec![
///         ContactStatus {
///             status: "clientTransferProhibited".to_string()
///         }
///     ];
///
///     contact_update.add(add_statuses);
///
///     // send it to the registry and receive a response of type EppContactUpdateResponse
///     let response = client.transact::<_, EppContactUpdateResponse>(&contact_update).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppContactUpdate = EppObject<Command<ContactUpdate>>;

/// Type for elements under the &lt;chg&gt; tag for contact update request
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

/// Type for list of elements of the &lt;status&gt; tag for contact update request
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusList {
    status: Vec<ContactStatus>,
}

/// Type for elements under the contact &lt;update&gt; tag
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
/// Type for EPP XML &lt;update&gt; command for contacts
pub struct ContactUpdate {
    /// The data under the &lt;update&gt; tag for the contact update
    #[serde(rename = "update")]
    contact: ContactUpdateData,
}

impl ContactUpdate {
    pub fn new(id: &str) -> ContactUpdate {
        ContactUpdate {
            contact: ContactUpdateData {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.to_string_value(),
                add_statuses: None,
                remove_statuses: None,
                change_info: None,
            },
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
        self.contact.change_info = Some(ContactChangeInfo {
            email: Some(email.to_string_value()),
            postal_info: Some(postal_info),
            voice: Some(voice),
            auth_info: Some(AuthInfo::new(auth_password)),
            fax: None,
        });
    }

    /// Sets the data for the &lt;fax&gt; tag under &lt;chg&gt; for the contact update request
    pub fn set_fax(&mut self, fax: Phone) {
        if let Some(info) = &mut self.contact.change_info {
            info.fax = Some(fax)
        }
    }

    /// Sets the data for the &lt;add&gt; tag for the contact update request
    pub fn add(&mut self, statuses: Vec<ContactStatus>) {
        self.contact.add_statuses = Some(StatusList { status: statuses });
    }

    /// Sets the data for the &lt;rem&gt; tag for the contact update request
    pub fn remove(&mut self, statuses: Vec<ContactStatus>) {
        self.contact.remove_statuses = Some(StatusList { status: statuses });
    }

    /// Loads data into the &lt;chg&gt; tag from an existing EppContactInfoResponse object
    pub fn load_from_contact_info(&mut self, contact_info: ContactInfoResult) {
        self.contact.change_info = Some(ContactChangeInfo {
            email: Some(contact_info.info_data.email.clone()),
            postal_info: Some(contact_info.info_data.postal_info.clone()),
            voice: Some(contact_info.info_data.voice.clone()),
            fax: contact_info.info_data.fax,
            auth_info: None,
        });
    }
}

impl EppContactUpdate {
    /// Creates a new EppObject for contact update corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(id: &str, client_tr_id: &str) -> EppContactUpdate {
        let contact_update = ContactUpdate::new(id);
        EppObject::build(Command::<ContactUpdate>::new(contact_update, client_tr_id))
    }

    /// Sets the data for the &lt;chg&gt; tag for the contact update request
    pub fn set_info(
        &mut self,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
    ) {
        self.data
            .command
            .set_info(email, postal_info, voice, auth_password);
    }

    /// Sets the data for the &lt;fax&gt; tag under &lt;chg&gt; for the contact update request
    pub fn set_fax(&mut self, fax: Phone) {
        self.data.command.set_fax(fax);
    }

    /// Sets the data for the &lt;add&gt; tag for the contact update request
    pub fn add(&mut self, statuses: Vec<ContactStatus>) {
        self.data.command.add(statuses);
    }

    /// Sets the data for the &lt;rem&gt; tag for the contact update request
    pub fn remove(&mut self, statuses: Vec<ContactStatus>) {
        self.data.command.remove(statuses);
    }

    /// Loads data into the &lt;chg&gt; tag from an existing EppContactInfoResponse object
    pub fn load_from_epp_contact_info(
        &mut self,
        contact_info: EppContactInfoResponse,
    ) -> Result<(), error::Error> {
        match contact_info.data.res_data {
            Some(res_data) => {
                self.data.command.load_from_contact_info(res_data);
                Ok(())
            }
            None => Err(error::Error::Other(
                "No res_data in EppContactInfoResponse object".to_string(),
            )),
        }
    }
}
