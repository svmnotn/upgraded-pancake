use crate::{error::Error, table::Table};
use serde_derive::Serialize;
use upgraded_pancake::TableResult;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response {
    Table(Table),
    Probability(f64),
    Probabilities(Vec<f64>),
    Roll(TableResult),
    Error(Error),
    Status(u8),
}

impl From<Table> for Response {
    fn from(t: Table) -> Self {
        Response::Table(t)
    }
}

impl From<f64> for Response {
    fn from(p: f64) -> Self {
        Response::Probability(p)
    }
}

impl From<Vec<f64>> for Response {
    fn from(p: Vec<f64>) -> Self {
        Response::Probabilities(p)
    }
}

impl From<TableResult> for Response {
    fn from(t: TableResult) -> Self {
        Response::Roll(t)
    }
}

impl From<Error> for Response {
    fn from(e: Error) -> Self {
        Response::Error(e)
    }
}

impl From<u8> for Response {
    fn from(s: u8) -> Self {
        Response::Status(s)
    }
}
