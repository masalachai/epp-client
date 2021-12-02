use std::fmt::Debug;

use epp_client_macros::ElementName;
use serde::{Deserialize, Serialize};

use crate::{
    common::{ElementName, NoExtension},
    request::{EppExtension, EppRequest},
    response::ResponseStatus,
};

#[derive(Debug)]
pub struct Logout<E> {
    request: LogoutRequest,
    extension: Option<E>,
}

impl<E: EppExtension> EppRequest<E> for Logout<E> {
    type Input = LogoutRequest;
    type Output = ResponseStatus;

    fn into_parts(self) -> (Self::Input, Option<E>) {
        (self.request, self.extension)
    }
}

impl<E: EppExtension> Logout<E> {
    pub fn new() -> Logout<NoExtension> {
        Logout {
            request: LogoutRequest {},
            extension: None,
        }
    }

    pub fn with_extension<F: EppExtension>(self, extension: F) -> Logout<F> {
        Logout {
            request: self.request,
            extension: Some(extension),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ElementName)]
#[element_name(name = "logout")]
/// Type corresponding to the &lt;logout&gt; tag in an EPP XML logout request
pub struct LogoutRequest;
