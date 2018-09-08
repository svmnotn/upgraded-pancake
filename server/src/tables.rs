use crate::error::*;
use rand::{thread_rng, Rng};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;
use upgraded_pancake::Table;

fn from_cookies(name: &str, cookies: &Cookies) -> Result {
    match cookies
        .get(name)
        .ok_or_else(|| Error::TableNotFound(String::from(name)))
        .and_then(|c| base64::decode(c.value()).map_err(Into::into))
        .and_then(|b| serde_json::from_slice(&b).map_err(Into::into))
    {
        Ok(t) => Result::Table(t),
        Err(e) => Result::Error(e),
    }
}

fn from_str(table: &str) -> Result {
    match serde_json::from_str(table) {
        Ok(t) => Result::Table(t),
        Err(e) => Result::Error(e.into()),
    }
}

#[put(
    "/table/<name>",
    format = "application/json",
    data = "<table>"
)]
fn put(name: String, table: String, mut cookies: Cookies) -> Json<Result> {
    let t = from_str(&table);

    Json(if t.is_err() {
        t
    } else {
        cookies.add(Cookie::new(name, base64::encode(&table)));

        Result::Sucess
    })
}

#[get("/table/<name>")]
fn get(name: String, cookies: Cookies) -> Json<Result> {
    Json(from_cookies(&name, &cookies))
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
fn table_data(cookies: Cookies) -> Json<Vec<Result>> {
    Json(
        cookies
            .iter()
            .map(|c| from_cookies(c.name(), &cookies))
            .collect(),
    )
}

#[get("/table/<name>/roll")]
fn roll_saved(name: String, cookies: Cookies) -> Json<Result> {
    let table = from_cookies(&name, &cookies);
    Json(if let Result::Table(t) = table {
        if let Some(r) = t.roll() {
            Result::Roll(r)
        } else {
            Result::Error(Error::RollNotFound)
        }
    } else {
        table
    })
}

#[post(
    "/table/validate",
    format = "application/json",
    data = "<table>"
)]
fn validate(table: String) -> Json<Result> {
    Json(if let Result::Error(e) = from_str(&table) {
        Result::Error(e)
    } else {
        Result::Sucess
    })
}

#[post("/table", format = "application/json", data = "<table>")]
fn roll(table: String) -> Json<Result> {
    let t = from_str(&table);

    Json(if t.is_err() {
        t
    } else {
        if let Result::Table(ta) = t {
            if let Some(r) = ta.roll() {
                Result::Roll(r)
            } else {
                Result::Error(Error::RollNotFound)
            }
        } else {
            Result::Error(Error::TableNotFound(table))
        }
    })
}

#[get("/table/static")]
fn static_tables() -> Json<Table> {
    Json(
        serde_json::from_str(thread_rng().choose(&CHOICES).expect("choices empty?"))
            .expect("malformed test json"),
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
