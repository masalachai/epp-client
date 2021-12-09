use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    common::NoExtension,
    request::{Command, Transaction},
};

impl Transaction<NoExtension> for Logout {}

impl Command for Logout {
    type Response = ();
    const COMMAND: &'static str = "logout";
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Type corresponding to the &lt;logout&gt; tag in an EPP XML logout request
pub struct Logout;
