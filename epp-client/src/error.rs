//! Error types to wrap internal errors and make EPP errors easier to read

use crate::epp::response::EppCommandResponseError;
use std::fmt::Display;

/// Error enum holding the possible error types
#[derive(Debug)]
pub enum Error {
    EppConnectionError(std::io::Error),
    EppCommandError(EppCommandResponseError),
    EppDeserializationError(String),
    Other(String),
}

/// An EPP XML error
#[derive(Debug)]
pub struct EppCommandError {
    pub epp_error: EppCommandResponseError,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EppCommandError(e) => {
                write!(f, "epp-client EppCommandError: {}", e.data.result.message)
            }
            Error::Other(e) => write!(f, "epp-client Exception: {}", e),
            _ => write!(f, "epp-client Exception: {:?}", self),
        }
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for Error {
    fn from(e: std::boxed::Box<dyn std::error::Error>) -> Self {
        Self::Other(format!("{:?}", e).to_string())
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
