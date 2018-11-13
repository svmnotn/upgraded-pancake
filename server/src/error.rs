use base64::DecodeError as Base64Error;
use serde_derive::Serialize;
use serde_json::error::Error as SerdeError;
use std::fmt;

#[derive(Debug, Serialize)]
#[serde(tag = "error", content = "data")]
pub enum Error {
    Serde(String),
    /// This one would only occur if someone
    /// *else* messed with the cookies.
    Base64(String),
    TableNotFound(String),
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self {
        Error::Serde(format!("{}", e))
    }
}

impl From<Base64Error> for Error {
    fn from(e: Base64Error) -> Self {
        Error::Base64(format!("{}", e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serde(e) => write!(f, "{}", e),
            Error::Base64(e) => write!(f, "{}", e),
            Error::TableNotFound(e) => write!(f, "{}", e),
        }
    }
}
