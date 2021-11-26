//! Types for EPP requests and responses

pub mod object;
pub mod request;
pub mod response;
pub mod xml;

pub use request::contact::check::*;
pub use request::contact::create::*;
pub use request::contact::delete::*;
pub use request::contact::info::*;
pub use request::contact::update::*;
pub use request::domain::rgp::report::*;
pub use request::domain::rgp::request::*;
pub use request::domain::transfer::*;
pub use request::domain::update::*;
pub use request::host::check::*;
pub use request::host::create::*;
pub use request::host::delete::*;
pub use request::host::info::*;
pub use request::host::update::*;
pub use request::message::ack::*;
pub use request::message::poll::*;

pub use response::contact::check::*;
pub use response::contact::create::*;
pub use response::contact::delete::*;
pub use response::contact::info::*;
pub use response::contact::update::*;
pub use response::domain::rgp::report::*;
pub use response::domain::rgp::request::*;
pub use response::domain::transfer::*;
pub use response::domain::update::*;
pub use response::host::check::*;
pub use response::host::create::*;
pub use response::host::delete::*;
pub use response::host::info::*;
pub use response::host::update::*;
pub use response::message::ack::*;
pub use response::message::poll::*;

pub use crate::connection::client::default_client_tr_id_fn as generate_client_tr_id;
