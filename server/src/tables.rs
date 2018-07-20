extern crate rand;
extern crate serde_json;

use rocket_contrib::Json;
use upgraded_pancake::{Table, TableResult};

const SIMPLE_TABLE_SRC: &'static str = r#"
{
  "die": "1d6",
  "results": [
    {
      "roll": 1,
      "value": "DATA"
    },
    {
      "roll": 2,
      "value": "DATA1"
    },
    {
      "roll": 3,
      "value": "DATA2"
    }
  ]
}
"#;

const COMPLEX_TABLE_SRC: &'static str = r#"
{
  "die": {
    "amount": 3,
    "size": 6
  },
  "results": [
    {
      "range": "3-6",
      "value": "DATA"
    },
    {
      "range": "7-11",
      "value": "DATA1"
    },
    {
      "range": "12-18",
      "value": "DATA2"
    }
  ]
}
"#;

#[get("/table")]
fn get() -> Json<Table> {
    Json(rand::random())
}

#[post("/table", format = "application/json", data = "<table>")]
fn post(table: Option<Json<Table>>) -> Option<Json<TableResult>> {
    table.and_then(|t| t.0.get()).map(|v| Json(v))
}

#[get("/table/simple")]
fn get_simple() -> Option<Json<TableResult>> {
    serde_json::from_str::<Table>(SIMPLE_TABLE_SRC)
        .unwrap()
        .get()
        .map(|v| Json(v))
}

#[get("/table/complex")]
fn get_complex() -> Option<Json<TableResult>> {
    serde_json::from_str::<Table>(COMPLEX_TABLE_SRC)
        .unwrap()
        .get()
        .map(|v| Json(v))
}
