#[derive(Serialize, Deserialize)]
pub struct Table {
  die: Die,
  results: Vec<Row>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Die {
    Simple {
        value: String,
    },
    Complex {
        amount: i64,
        size: i64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Row {
    Simple {
        roll: i64,
        value: String,
    },
    Complex {
        range: String, // TODO convert into Range
        value: String,
    },
}
