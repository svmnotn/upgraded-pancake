extern crate rand;
extern crate serde_json;
use self::rand::{thread_rng, Rng};
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

#[get("/table/static")]
fn get_static() -> Json<Table> {
    Json(
        serde_json::from_str(thread_rng().choose(&CHOICES).expect("choices empty?"))
            .expect("wrong json"),
    )
}

const CHOICES: [&'static str; 5] = [
    r#"{"dice":"1d6","results":[{"roll":1,"value":"DATA"},{"roll":2,"value":"DATA1"},{"roll":3,"value":"DATA2"},{"roll":4,"value":"DATA3"},{"roll":5,"value":"DATA4"},{"roll":6,"value":"DATA5"}]}"#,
    r#"{"dice":{"amount":2,"size":6},"results":[{"roll":2,"value":"DATA"},{"range":"2-4","value":"DATA1"},{"roll":5,"value":"DATA3"},{"range":"6-10","value":"DATA4"},{"roll":11,"value":"DATA5"},{"roll":12,"value":"DATA6"}]}"#,
    r#"{"dice":{"amount":1,"size":10},"results":[{"range":"1-3","value":"DATA"},{"roll":4,"value":"DATA1"},{"roll":5,"value":"DATA3"},{"roll":6,"value":"DATA4"},{"roll":7,"value":"DATA5"},{"range":"8-9","value":"DATA6"},{"roll":10,"value":"DATA6"}]}"#,
    r#"{"dice":"3d4","results":[{"range":"3-4","value":"DATA"},{"range":"5-6","value":"DATA1"},{"range":"7-8","value":"DATA3"},{"range":"9-10","value":"DATA4"},{"range":"11-12","value":"DATA5"}]}"#,
    r#"{"dice":"1d4","results":[{"roll":1,"value":"DATA"},{"roll":2,"value":"DATA1"},{"roll":3,"value":"DATA3"},{"roll":4,"value":"DATA4"}]}"#,
];
