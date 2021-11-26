//! Types for EPP requests and responses

pub mod object;
pub mod request;
pub mod response;
pub mod xml;

pub use request::host::update::*;
pub use request::message::ack::*;
pub use request::message::poll::*;

pub use response::host::update::*;
pub use response::message::ack::*;
pub use response::message::poll::*;

pub use crate::connection::client::default_client_tr_id_fn as generate_client_tr_id;
