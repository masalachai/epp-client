//! Types for EPP contact info request for neustar

use crate::epp::request::contact::info::EppContactInfo;

/// Type for the &lt;epp&gt; request for contact &lt;info&gt; command for Neustar
///
/// ## Usage
///
/// ```ignore
/// use epp_client::EppClient;
/// use epp_client::epp::{EppNeustarContactInfo, EppNeustarContactInfoResponse};
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
///     // Create an EppNeustarContactInfo instance
///     let contact_info = EppNeustarContactInfo::new(
///         "eppdev-contact-100",
///         "epP4uthd#v",
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppNeustarContactInfoResponse
///     let response = client.transact::<_, EppNeustarContactInfoResponse>(&contact_info).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppNeustarContactInfo = EppContactInfo;
