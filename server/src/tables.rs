use crate::error::*;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::Json;
use upgraded_pancake::Table;

fn from_cookies(name: &str, cookies: &Cookies) -> Response {
    match cookies
        .get(name)
        .ok_or_else(|| Error::TableNotFound(String::from(name)))
        .and_then(|c| base64::decode(c.value()).map_err(Into::into))
        .and_then(|b| serde_json::from_slice::<Table>(&b).map_err(Into::into))
    {
        Ok(t) => t.into(),
        Err(e) => e.into(),
    }
}

fn from_str(table: &str) -> Response {
    match serde_json::from_str::<Table>(table) {
        Ok(t) => t.into(),
        Err(e) => Response::Error(e.into()),
    }
}

#[put("/<name>", format = "application/json", data = "<table>")]
pub fn put(name: String, table: String, mut cookies: Cookies) -> Json<Response> {
    Json(match from_str(&table) {
        Response::Table(_) => {
            cookies.add(Cookie::new(name, base64::encode(&table)));
            Response::Status(0)
        }
        Response::Error(e) => e.into(),
        _ => unreachable!("Got something other than a table / error"),
    })
}

#[get("/<name>")]
pub fn get(name: String, cookies: Cookies) -> Json<Response> {
    Json(from_cookies(&name, &cookies))
}

#[delete("/<name>")]
pub fn delete(name: String, mut cookies: Cookies) {
    cookies.remove(Cookie::named(name));
}

#[get("/all/id")]
pub fn table_ids(cookies: Cookies) -> Json<Vec<String>> {
    Json(cookies.iter().map(|c| c.name().to_owned()).collect())
}

#[get("/all")]
pub fn all(cookies: Cookies) -> Json<Vec<Response>> {
    Json(
        cookies
            .iter()
            .map(|c| from_cookies(c.name(), &cookies))
            .collect(),
    )
}

#[get("/<name>/roll")]
pub fn roll_saved(name: String, cookies: Cookies) -> Json<Response> {
    Json(match from_cookies(&name, &cookies) {
        Response::Table(t) => t.roll().into(),
        Response::Error(e) => e.into(),
        _ => unreachable!("Got something other than a table / error"),
    })
}

#[post("/validate", format = "application/json", data = "<table>")]
pub fn validate(table: String) -> Json<Response> {
    Json(match from_str(&table) {
        Response::Error(e) => e.into(),
        _ => Response::Status(0),
    })
}

#[post("/", format = "application/json", data = "<table>")]
pub fn roll(table: String) -> Json<Response> {
    Json(match from_str(&table) {
        Response::Error(e) => e.into(),
        Response::Table(t) => t.roll().into(),
        _ => unreachable!("Got something other than a table / error"),
    })
}

#[get("/static")]
pub fn static_tables() -> Json<Table> {
    use rand::seq::SliceRandom;
    Json(
        serde_json::from_str(
            CHOICES
                .choose(&mut rand::thread_rng())
                .expect("choices empty?"),
        )
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
