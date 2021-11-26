//! Types for EPP message poll request

use epp_client_macros::*;

use crate::common::{ElementName, EppObject, StringValue};
use crate::request::Command;
use crate::response::CommandResponse;
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
/// use epp_client::message::poll::{EppMessagePoll, EppMessagePollResponse};
/// use epp_client::generate_client_tr_id;
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
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppMessagePoll = EppObject<Command<MessagePollRequest>>;

impl EppMessagePoll {
    /// Creates a new EppObject for &lt;poll&gt; req corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(client_tr_id: &str) -> EppMessagePoll {
        EppObject::build(Command::<MessagePollRequest>::new(
            MessagePollRequest {
                op: "req".to_string(),
            },
            client_tr_id,
        ))
    }
}

/// Type that represents the &lt;epp&gt; tag for the EPP XML message poll response
pub type EppMessagePollResponse = EppObject<CommandResponse<MessagePollResponse>>;

// Request

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "poll")]
/// Type for EPP XML &lt;poll&gt; command for message poll
pub struct MessagePollRequest {
    /// The type of operation to perform
    /// The value is "req" for message polling
    op: String,
}

// Response

/// Type that represents the &lt;trnData&gt; tag for message poll response
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDomainTransferData {
    /// XML namespace for message response data
    #[serde(rename = "xmlns:obj", alias = "xmlns")]
    xmlns: String,
    /// The name of the domain under transfer
    #[serde(rename = "obj:name", alias = "name")]
    pub name: StringValue,
    /// The domain transfer status
    #[serde(rename = "obj:trStatus", alias = "trStatus")]
    pub transfer_status: StringValue,
    /// The epp user who requested the transfer
    #[serde(rename = "obj:reID", alias = "reID")]
    pub requester_id: StringValue,
    /// The date of the transfer request
    #[serde(rename = "obj:reDate", alias = "reDate")]
    pub requested_at: StringValue,
    /// The epp user who should acknowledge the transfer request
    #[serde(rename = "obj:acID", alias = "acID")]
    pub ack_id: StringValue,
    /// The date by which the transfer request should be acknowledged
    #[serde(rename = "obj:acDate", alias = "acDate")]
    pub ack_by: StringValue,
    /// The domain expiry date
    #[serde(rename = "obj:exDate", alias = "exDate")]
    pub expiring_at: StringValue,
}

/// Type that represents the &lt;resData&gt; tag for message poll response
#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePollResponse {
    /// Data under the &lt;trnData&gt; tag
    #[serde(rename = "obj:trnData", alias = "trnData")]
    pub message_data: MessageDomainTransferData,
}
