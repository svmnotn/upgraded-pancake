#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Strings {
    Single(String),
    Multiple(Vec<String>),
}
