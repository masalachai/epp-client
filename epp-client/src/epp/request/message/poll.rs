//! Types for EPP message poll request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject};
use crate::epp::request::Command;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for registry <poll op="req"> command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppMessagePoll, EppMessagePollResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an EppMessagePoll instance
///     let message_poll = EppMessagePoll::new(generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppMessagePollResponse
///     let response = client.transact::<_, EppMessagePollResponse>(&message_poll).await.unwrap();
///
///     println!("{:?}", response);
/// }
/// ```
pub type EppMessagePoll = EppObject<Command<MessagePoll>>;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message poll
pub struct MessagePoll {
    /// The type of operation to perform
    /// The value is "req" for message polling
    op: String,
}

impl EppMessagePoll {
    /// Creates a new EppObject for &lt;poll&gt; req corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(client_tr_id: &str) -> EppMessagePoll {
        EppObject::build(Command::<MessagePoll>::new(
            MessagePoll {
                op: "req".to_string(),
            },
            client_tr_id,
        ))
    }
}
