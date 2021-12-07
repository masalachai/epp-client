//! Error types to wrap internal errors and make EPP errors easier to read

use std::error::Error as StdError;
use std::fmt::Display;

use crate::response::ResponseStatus;

/// Error enum holding the possible error types
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Command(ResponseStatus),
    Xml(Box<dyn StdError>),
    Other(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Command(e) => {
                write!(f, "epp-client EppCommandError: {}", e.result.message)
            }
            Error::Other(e) => write!(f, "epp-client Exception: {}", e),
            _ => write!(f, "epp-client Exception: {:?}", self),
        }
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for Error {
    fn from(e: std::boxed::Box<dyn std::error::Error>) -> Self {
        Self::Other(format!("{:?}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::io::ErrorKind> for Error {
    fn from(e: std::io::ErrorKind) -> Self {
        Self::Io(std::io::Error::from(e))
    }
}
