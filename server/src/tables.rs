use crate::{error::*, response::Response, table::Table};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::Json;

fn from_cookies(id: &str, cookies: &Cookies) -> Response {
    match cookies
        .get(id)
        .ok_or_else(|| Error::TableNotFound(String::from(id)))
        .and_then(|c| base64::decode(c.value()).map_err(Into::into))
        .and_then(|b| serde_json::from_slice::<Table>(&b).map_err(Into::into))
    {
        Ok(t) => t.fill_in().into(),
        Err(e) => e.into(),
    }
}

fn from_str(table: &str) -> Response {
    match serde_json::from_str::<Table>(table) {
        Ok(t) => t.fill_in().into(),
        Err(e) => Response::Error(e.into()),
    }
}

#[put("/<id>", format = "application/json", data = "<table>")]
pub fn put(id: String, table: String, mut cookies: Cookies) -> Json<Response> {
    Json(match from_str(&table) {
        Response::Table(t) => match serde_json::to_string(&t).map_err(|e| Error::from(e)) {
            Ok(t) => {
                cookies.add(Cookie::new(id, base64::encode(&t)));
                Response::Status(0)
            }
            Err(e) => e.into(),
        },
        Response::Error(e) => e.into(),
        _ => unreachable!("Got something other than a table / error"),
    })
}

#[get("/<id>")]
pub fn get(id: String, cookies: Cookies) -> Json<Response> {
    Json(from_cookies(&id, &cookies))
}

#[delete("/<id>")]
pub fn delete(id: String, mut cookies: Cookies) {
    cookies.remove(Cookie::named(id));
}

#[get("/<id>/roll")]
pub fn roll_saved(id: String, cookies: Cookies) -> Json<Response> {
    Json(match from_cookies(&id, &cookies) {
        Response::Table(t) => t.roll().into(),
        Response::Error(e) => e.into(),
        _ => unreachable!("Got something other than a table / error"),
    })
}

#[get("/<id>/probability")]
pub fn probability(id: String, cookies: Cookies) -> Json<Response> {
    Json(match from_cookies(&id, &cookies) {
        Response::Table(t) => t.probabilities().into(),
        Response::Error(e) => e.into(),
        _ => unreachable!("Got something other than a table / error"),
    })
}

#[get("/<id>/probability/<row>")]
pub fn probability_of_row(id: String, row: u32, cookies: Cookies) -> Json<Response> {
    Json(match from_cookies(&id, &cookies) {
        Response::Table(t) => t.probability(row).into(),
        Response::Error(e) => e.into(),
        _ => unreachable!("Got something other than a table / error"),
    })
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

#[get("/all/id")]
pub fn table_ids(cookies: Cookies) -> Json<Vec<String>> {
    Json(cookies.iter().map(|c| c.name().to_owned()).collect())
}

#[post("/", format = "application/json", data = "<table>")]
pub fn roll(table: String) -> Json<Response> {
    Json(match from_str(&table) {
        Response::Error(e) => e.into(),
        Response::Table(t) => t.roll().into(),
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
    include_str!("../../test_data/lib/simple/value_single.json"),
    include_str!("../../test_data/lib/simple/value_multiple.json"),
    include_str!("../../test_data/lib/simple/ranges_single.json"),
    include_str!("../../test_data/lib/simple/ranges_multiple.json"),
    include_str!("../../test_data/lib/simple/mixed_single.json"),
    include_str!("../../test_data/lib/simple/mixed_multiple.json"),
    include_str!("../../test_data/lib/complex/value_single.json"),
    include_str!("../../test_data/lib/complex/value_multiple.json"),
    include_str!("../../test_data/lib/complex/ranges_single.json"),
    include_str!("../../test_data/lib/complex/ranges_multiple.json"),
    include_str!("../../test_data/lib/complex/mixed_single.json"),
    include_str!("../../test_data/lib/complex/mixed_multiple.json"),
];
