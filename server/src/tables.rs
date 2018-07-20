extern crate rand;
extern crate serde_json;

use rocket_contrib::Json;
use upgraded_pancake::{Table, TableResult};

#[get("/table")]
fn get() -> Json<Table> {
    Json(rand::random())
}

#[post("/table", format = "application/json", data = "<table>")]
fn post(table: Option<Json<Table>>) -> Option<Json<TableResult>> {
    table.and_then(|t| t.0.get()).map(|v| Json(v))
}
