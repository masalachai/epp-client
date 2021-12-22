//! Error types to wrap internal errors and make EPP errors easier to read

use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io;

use crate::response::ResponseStatus;

/// Error enum holding the possible error types
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Command(ResponseStatus),
    Xml(Box<dyn StdError>),
    Other(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Command(e) => {
                write!(f, "epp-client EppCommandError: {}", e.result.message)
            }
            Error::Other(e) => write!(f, "epp-client Exception: {}", e),
            _ => write!(f, "epp-client Exception: {:?}", self),
        }
    }
}

impl From<Box<dyn StdError>> for Error {
    fn from(e: Box<dyn StdError>) -> Self {
        Self::Other(format!("{:?}", e))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<io::ErrorKind> for Error {
    fn from(e: io::ErrorKind) -> Self {
        Self::Io(io::Error::from(e))
    }
}
