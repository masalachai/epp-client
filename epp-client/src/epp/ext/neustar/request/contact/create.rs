//! Types for EPP contact create request with neustar extension

use crate::epp::ext::neustar::object::data::ContactExtension;
use crate::epp::ext::neustar::object::data::Extension as NeustarExtension;
use crate::epp::ext::neustar::xml::{EPP_NEULEVEL_XMLNS, EPP_XSI_NEULEVEL_SCHEMA_LOCATION};
use crate::epp::object::data::{Phone, PostalInfo};
use crate::epp::object::{EppObject, Extension, StringValueTrait};
use crate::epp::request::contact::create::ContactCreate;
use crate::epp::request::CommandWithExtension;

/// Type that represents the &lt;epp&gt; request for contact &lt;create&gt; command for Neustar
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::object::data::{Address, Phone, PostalInfo};
/// use epp_client::epp::{EppNeustarContactCreate, EppNeustarContactCreateResponse};
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
///     // Create the address, postal_info, voice instances
///     let street = vec!["58", "Orchid Road"];
///     let address = Address::new(street, "New York", "New York", "392374", "US");
///     let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
///     let mut voice = Phone::new("+1.47237942");
///     voice.set_extension("123");
///     let mut fax = Phone::new("+1.86698799");
///     fax.set_extension("677");
///
///     let ext = ContactExtension {
///         ext_contact: None,
///         app_purpose: Some("P2".to_string()),
///         nexus_category: Some("C31/DE".to_string()),
///     };
///
///     // Create an EppNeustarContactCreate instance
///     let mut contact_create = EppNeustarContactCreate::new(
///         "eppdev-contact-100",
///         "contact@eppdev.net",
///         postal_info,
///         voice,
///         "epP4uthd#v",
///         ext,
///         generate_client_tr_id(&client).as_str()
///     );
///     contact_create.set_fax(fax);
///
///     // send it to the registry and receive a response of type EppNeustarContactCreateResponse
///     let response = client.transact::<_, EppNeustarContactCreateResponse>(&contact_create).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppNeustarContactCreate = EppObject<CommandWithExtension<ContactCreate, NeustarExtension>>;

impl EppNeustarContactCreate {
    /// Creates a new EppObject for contact create corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(
        id: &str,
        email: &str,
        postal_info: PostalInfo,
        voice: Phone,
        auth_password: &str,
        extension: ContactExtension,
        client_tr_id: &str,
    ) -> EppNeustarContactCreate {
        let contact_create = ContactCreate::new(id, email, postal_info, voice, auth_password);

        let ext = NeustarExtension {
            xmlns: EPP_NEULEVEL_XMLNS.to_string(),
            schema_location: EPP_XSI_NEULEVEL_SCHEMA_LOCATION.to_string(),
            unspec: extension.to_string().to_string_value(),
        };

        EppObject::build(CommandWithExtension::<_, NeustarExtension> {
            command: contact_create,
            extension: Some(Extension { data: ext }),
            client_tr_id: client_tr_id.to_string_value(),
        })
    }

    /// Sets the &lt;fax&gt; data for the request
    pub fn set_fax(&mut self, fax: Phone) {
        self.data.command.contact.set_fax(fax);
    }
}
