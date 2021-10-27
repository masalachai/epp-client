//! Types for EPP contact create request

use epp_client_macros::*;

use crate::epp::object::data;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for contact &lt;create&gt; command
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::object::data::{Address, Phone, PostalInfo};
/// use epp_client::epp::{EppContactCreate, EppContactCreateResponse};
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
///     // Create an EppContactCreate instance
///     let mut contact_create = EppContactCreate::new(
///         "eppdev-contact-100",
///         "contact@eppdev.net",
///         postal_info,
///         voice,
///         "epP4uthd#v",
///         generate_client_tr_id(&client).as_str()
///     );
///     contact_create.set_fax(fax);
///
///     // send it to the registry and receive a response of type EppContactCreateResponse
///     let response = client.transact::<_, EppContactCreateResponse>(&contact_create).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppContactCreate = EppObject<Command<ContactCreate>>;

/// Type for elements under the contact &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    /// XML namespace for contact commands
    xmlns: String,
    /// Contact &lt;id&gt; tag
    id: StringValue,
    /// Contact &lt;postalInfo&gt; tag
    #[serde(rename = "postalInfo")]
    postal_info: data::PostalInfo,
    /// Contact &lt;voice&gt; tag
    voice: data::Phone,
    /// Contact &lt;fax&gt; tag,
    fax: Option<data::Phone>,
    /// Contact &lt;email&gt; tag
    email: StringValue,
    /// Contact &lt;authInfo&gt; tag
    #[serde(rename = "authInfo")]
    auth_info: data::AuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for contacts
pub struct ContactCreate {
    /// Data for &lt;create&gt; command for contact
    #[serde(rename = "create")]
    pub contact: Contact,
}

impl EppContactCreate {
    /// Creates a new EppObject for contact create corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(
        id: &str,
        email: &str,
        postal_info: data::PostalInfo,
        voice: data::Phone,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppContactCreate {
        let contact_create = ContactCreate {
            contact: Contact {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.to_string_value(),
                postal_info,
                voice,
                fax: None,
                email: email.to_string_value(),
                auth_info: data::AuthInfo::new(auth_password),
            },
        };

        EppObject::build(Command::<ContactCreate>::new(contact_create, client_tr_id))
    }

    /// Sets the &lt;fax&gt; data for the request
    pub fn set_fax(&mut self, fax: data::Phone) {
        self.data.command.contact.fax = Some(fax);
    }
}
