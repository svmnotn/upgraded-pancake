// TODO better tests

use crate::rocket;
use rocket::http::ContentType;
use rocket::local::Client;

#[test]
fn can_add_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/complex/value_single.json"))
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());
}
