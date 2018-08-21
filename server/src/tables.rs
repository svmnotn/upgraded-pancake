use rand::{thread_rng, Rng};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;
use upgraded_pancake::{Table, TableResult};

fn get_table(name: &str, cookies: &Cookies) -> Option<Table> {
    cookies
        .get(name)
        .and_then(|c| base64::decode(c.value()).ok())
        .and_then(|b| serde_json::from_slice(&b).ok())
}

#[put(
    "/table/<name>",
    format = "application/json",
    data = "<table>"
)]
fn put(name: String, table: Json<Table>, mut cookies: Cookies) -> Json<bool> {
    Json(if table.is_valid() {
        cookies.add(Cookie::new(
            name,
            base64::encode(&serde_json::to_string(&table.0).expect("Unable to JSONify JSON?")),
        ));
        true
    } else {
        false
    })
}

#[get("/table/<name>")]
fn get(name: String, cookies: Cookies) -> Option<Json<Table>> {
    get_table(&name, &cookies).map(Json)
}

#[delete("/table/<name>")]
fn delete(name: String, mut cookies: Cookies) {
    cookies.remove(Cookie::named(name));
}

#[get("/table/all/name")]
fn table_name(cookies: Cookies) -> Json<Vec<String>> {
    Json(cookies.iter().map(|c| c.name().to_owned()).collect())
}

#[get("/table/all/data")]
fn table_data(cookies: Cookies) -> Json<Vec<Table>> {
    Json(
        cookies
            .iter()
            .map(|c| get_table(c.name(), &cookies))
            .filter(Option::is_some)
            // The following is only ok because of the filter above
            .map(Option::unwrap)
            .collect(),
    )
}

#[get("/table/<name>/roll")]
fn roll_saved(name: String, cookies: Cookies) -> Option<Json<TableResult>> {
    match get_table(&name, &cookies) {
        Some(ref t) if t.is_valid() => t.roll().map(Json),
        _ => None,
    }
}

#[post("/table", format = "application/json", data = "<table>")]
fn roll(table: Json<Table>) -> Option<Json<TableResult>> {
    if table.is_valid() {
        table.roll().map(Json)
    } else {
        None
    }
}

#[post(
    "/table/validate",
    format = "application/json",
    data = "<table>"
)]
fn validate(table: Json<Table>) -> Json<bool> {
    Json(table.is_valid())
}

#[get("/table/static")]
fn static_tables() -> Json<Table> {
    Json(
        serde_json::from_str(thread_rng().choose(&CHOICES).expect("choices empty?"))
            .expect("wrong json"),
    )
}

const CHOICES: [&str; 6] = [
    r#"{"dice":"1d6","heading":"Test Data","results":[{"roll":1,"value":"DATA"},{"roll":2,"value":"DATA1"},{"roll":3,"value":"DATA2"},{"roll":4,"value":"DATA3"},{"roll":5,"value":"DATA4"},{"roll":6,"value":"DATA5"}]}"#,
    r#"{"dice":{"amount":2,"size":6},"heading":"Complex? Test Data","results":[{"roll":2,"value":"DATA"},{"roll":"3-4","value":"DATA1"},{"roll":5,"value":"DATA3"},{"roll":"6-10","value":"DATA4"},{"roll":11,"value":"DATA5"},{"roll":12,"value":"DATA6"}]}"#,
    r#"{"dice":{"amount":1,"size":10},"heading":"Problematic? Test Data2","results":[{"roll":"1-3","value":"DATA"},{"roll":4,"value":"DATA1"},{"roll":5,"value":"DATA3"},{"roll":6,"value":"DATA4"},{"roll":7,"value":"DATA5"},{"roll":"8-9","value":"DATA6"},{"roll":10,"value":"DATA6"}]}"#,
    r#"{"dice":"3d4","heading":"Interesting? Test Data","results":[{"roll":"3-4","value":"DATA"},{"roll":"5-6","value":"DATA1"},{"roll":"7-8","value":"DATA3"},{"roll":"9-10","value":"DATA4"},{"roll":"11-12","value":"DATA5"}]}"#,
    r#"{"dice":"1d4","heading":"Hi it is Test Data","results":[{"roll":1,"value":"DATA"},{"roll":2,"value":"DATA1"},{"roll":3,"value":"DATA3"},{"roll":4,"value":"DATA4"}]}"#,
    r#"{"dice":"1d6","heading":["Cool things","Stuffy stuff","is it all a lie?"],"results":[{"roll":1,"value":["I am cake","but that's a lie","cus the cake is a lie"]},{"roll":2,"value":["jerky","is neat","very neat"]},{"roll":3,"value":["cookies","very neat","we must eat"]},{"roll":4,"value":["we do","what we must","because we cam"]},{"roll":5,"value":["we all","lift","together"]},{"roll":6,"value":["we lift","for the grind","together"]}]}"#,
];
