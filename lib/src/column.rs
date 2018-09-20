use serde_derive::{Deserialize, Serialize};

/// The value of a column on a `Table`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Column {
    #[doc(hidden)]
    Single(String),
    #[doc(hidden)]
    Multiple(Vec<String>),
}

impl From<String> for Column {
    fn from(s: String) -> Self {
        Column::Single(s)
    }
}

impl From<Vec<String>> for Column {
    fn from(s: Vec<String>) -> Self {
        Column::Multiple(s)
    }
}
