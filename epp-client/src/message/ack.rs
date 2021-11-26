//! Types for EPP message ack request

use epp_client_macros::*;

use crate::common::{ElementName, EppObject};
use crate::request::Command;
use crate::response::CommandResponse;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for registry <poll op="ack"> command
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::message::ack::{EppMessageAck, EppMessageAckResponse};
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
///     // Create an EppMessageAck instance
///     let message_ack = EppMessageAck::new(12345, generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppMessageAckResponse
///     let response = client.transact::<_, EppMessageAckResponse>(&message_ack).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppMessageAck = EppObject<Command<MessageAckRequest>>;

impl EppMessageAck {
    /// Creates a new EppObject for &lt;poll&gt; ack corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(message_id: u32, client_tr_id: &str) -> EppMessageAck {
        EppObject::build(Command::<MessageAckRequest>::new(
            MessageAckRequest {
                op: "ack".to_string(),
                message_id: message_id.to_string(),
            },
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML message ack response
pub type EppMessageAckResponse = EppObject<CommandResponse<String>>;

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAckRequest {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}
