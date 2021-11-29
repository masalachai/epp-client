use std::fmt::Debug;

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, EppObject},
    request::Command,
    response::EppCommandResponse,
};

/// The EPP Logout request
pub type EppLogout = EppObject<Command<Logout>>;

impl EppLogout {
    /// Creates a new EPP Logout request
    pub fn new(client_tr_id: &str) -> EppLogout {
        EppObject::build(Command::<Logout> {
            command: Logout,
            extension: None,
            client_tr_id: client_tr_id.into(),
        })
    }
}

/// An alias of `EppCommandResponse` received in response to a successful logout request
pub type EppLogoutResponse = EppCommandResponse;

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "logout")]
/// Type corresponding to the &lt;logout&gt; tag in an EPP XML logout request
pub struct Logout;
