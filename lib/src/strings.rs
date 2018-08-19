#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
pub enum Strings {
    Single(String),
    Multiple(Vec<String>),
}

impl From<String> for Strings {
    fn from(s: String) -> Self {
        Strings::Single(s)
    }
}

impl From<Vec<String>> for Strings {
    fn from(s: Vec<String>) -> Self {
        Strings::Multiple(s)
    }
}
