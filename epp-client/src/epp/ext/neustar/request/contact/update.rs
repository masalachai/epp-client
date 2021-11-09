//! Types for EPP contact create request with neustar extension

use crate::epp::ext::neustar::object::data::ContactExtension;
use crate::epp::ext::neustar::object::data::Extension as NeustarExtension;
use crate::epp::ext::neustar::xml::{EPP_NEULEVEL_XMLNS, EPP_XSI_NEULEVEL_SCHEMA_LOCATION};
use crate::epp::object::data::{ContactStatus, Phone, PostalInfo};
use crate::epp::object::{EppObject, Extension, StringValueTrait};
use crate::epp::request::contact::update::ContactUpdate;
use crate::epp::request::CommandWithExtension;
use crate::epp::response::contact::info::EppContactInfoResponse;
use crate::error;

/// Type that represents the &lt;epp&gt; request for contact &lt;update&gt; command for Neustar
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNeustarContactUpdate, EppNeustarContactUpdateResponse};
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
///     // Create an EppNeustarContactUpdate instance
///     let mut contact_update = EppNeustarContactUpdate::new(
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
///     let ext = ContactExtension {
///         ext_contact: None,
///         app_purpose: None,
///         nexus_category: Some("C31/DE".to_string()),
///     };
///
///     contact_update.set_extension(ext);
///
///     // send it to the registry and receive a response of type EppNeustarContactUpdateResponse
///     let response = client.transact::<_, EppNeustarContactUpdateResponse>(&contact_update).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppNeustarContactUpdate = EppObject<CommandWithExtension<ContactUpdate, NeustarExtension>>;

impl EppNeustarContactUpdate {
    /// Creates a new EppObject for contact update corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(id: &str, client_tr_id: &str) -> EppNeustarContactUpdate {
        let contact_update = ContactUpdate::new(id);
        EppObject::build(CommandWithExtension::<_, NeustarExtension> {
            command: contact_update,
            extension: None,
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the extension params for the contact update request
    pub fn set_extension(&mut self, extension: ContactExtension) {
        self.data.extension = Some(Extension {
            data: NeustarExtension {
                xmlns: EPP_NEULEVEL_XMLNS.to_string(),
                schema_location: EPP_XSI_NEULEVEL_SCHEMA_LOCATION.to_string(),
                unspec: extension.to_string().to_string_value(),
            },
        });
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
