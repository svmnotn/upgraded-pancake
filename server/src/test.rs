// TODO better tests

use crate::rocket;
use rocket::http::ContentType;
use rocket::local::Client;

#[test]
fn bad_get_put() {
    let client = Client::new(rocket()).unwrap();

    // Try to get a message with an ID that doesn't exist.
    let mut res = client
        .post("/table")
        .header(ContentType::JSON)
        .body(r#"{ "contents": "not a table" }"#)
        .dispatch();

    let body = res.body_string().unwrap();
}
