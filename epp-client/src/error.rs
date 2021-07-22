use crate::epp::response::EppCommandResponseError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    EppConnectionError(std::io::Error),
    EppCommandError(EppCommandError),
    Other(String),
}

#[derive(Debug)]
pub struct EppCommandError {
    pub epp_error: EppCommandResponseError,
}

impl std::error::Error for Error {}

impl std::error::Error for EppCommandError {}

impl Display for EppCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "epp-client EppCommandError: {}",
            self.epp_error.data.result.message
        )
    }
}

impl EppCommandError {
    pub fn new(epp_error: EppCommandResponseError) -> EppCommandError {
        EppCommandError {
            epp_error: epp_error,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "epp-client Exception: {:?}", self)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for Error {
    fn from(e: std::boxed::Box<dyn std::error::Error>) -> Self {
        Self::Other(format!("{}", e).to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::EppConnectionError(e)
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(e: std::io::ErrorKind) -> Self {
        Self::EppConnectionError(std::io::Error::from(e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::Other(e)
    }
}
