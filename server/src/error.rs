// TODO Cleanup

use base64::DecodeError as Base64Error;
use serde_json::error::Error as SerdeError;
use std::fmt;
use upgraded_pancake::{Table, TableResult};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Result {
    Table(Table),
    Roll(TableResult),
    Error(Error),
    Status(u8),
}

impl Result {
    pub fn is_err(&self) -> bool {
        match self {
            Result::Error(_) => true,
            _ => false,
        }
    }
}

impl From<Table> for Result {
    fn from(t: Table) -> Self {
        Result::Table(t)
    }
}

impl From<TableResult> for Result {
    fn from(t: TableResult) -> Self {
        Result::Roll(t)
    }
}

impl From<Error> for Result {
    fn from(e: Error) -> Self {
        Result::Error(e)
    }
}

impl From<u8> for Result {
    fn from(r: u8) -> Self {
        Result::Status(r)
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "error", content = "data")]
pub enum Error {
    Serde(String),
    Base64(String),
    TableNotFound(String),
    RollNotFound,
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
            Error::RollNotFound => write!(f, "For some reason your roll was not found"),
        }
    }
}
