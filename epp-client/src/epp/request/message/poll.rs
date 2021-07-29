//! Types for EPP message poll request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject};
use crate::epp::request::Command;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for registry <poll op="req"> command
///
/// ## Usage
///
/// ```rust
/// use epp_client::EppClient;
/// use epp_client::epp::{EppMessagePoll, EppMessagePollResponse};
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
