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

const CHOICES: [&str; 12] = [
    include_str!("../../test_data/simple/value_single.json"),
    include_str!("../../test_data/simple/value_multiple.json"),
    include_str!("../../test_data/simple/ranges_single.json"),
    include_str!("../../test_data/simple/ranges_multiple.json"),
    include_str!("../../test_data/simple/mixed_single.json"),
    include_str!("../../test_data/simple/mixed_multiple.json"),
    include_str!("../../test_data/complex/value_single.json"),
    include_str!("../../test_data/complex/value_multiple.json"),
    include_str!("../../test_data/complex/ranges_single.json"),
    include_str!("../../test_data/complex/ranges_multiple.json"),
    include_str!("../../test_data/complex/mixed_single.json"),
    include_str!("../../test_data/complex/mixed_multiple.json"),
];
