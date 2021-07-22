use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    EppConnectionError(std::io::Error),
    Other(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "epp-client Exception: {:?}", self)
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

// impl From<std::io::Error> for Box<EppClientError> {
//     fn from(e: std::io::Error) -> Self {

//     }
// }
