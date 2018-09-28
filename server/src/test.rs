use crate::rocket;
use rocket::http::ContentType;
use rocket::local::Client;
use upgraded_pancake::{Table, TableResult};

const SIMPLE_TEST_DATA: &str = include_str!("../../test_data/simple/value_single.json");

#[test]
fn can_add_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());
}

#[test]
fn can_get_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client.get("/table/test").dispatch();

    assert_eq!(
        serde_json::from_str::<Table>(SIMPLE_TEST_DATA).expect("Failed to build SIMPLE_TEST_DATA"),
        serde_json::from_str::<Table>(&res.body_string().expect("Failed to get a Body"))
            .expect("Failed to build body")
    );
}

#[test]
fn can_remove_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let _ = client.delete("/table/test").dispatch();
}

#[test]
fn can_roll_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client.get("/table/test/roll").dispatch();

    assert!(
        serde_json::from_str::<TableResult>(&res.body_string().expect("Failed to get a Body"))
            .is_ok()
    );
}

#[test]
fn can_get_all_names() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .put("/table/test1")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client.get("/table/all/id").dispatch();

    let mut vals =
        serde_json::from_str::<Vec<String>>(&res.body_string().expect("Failed to get a Body"))
            .expect("Failed to build body");
    vals.sort_unstable();

    assert_eq!(vec!["test", "test1"], vals);
}

#[test]
fn can_get_all_data() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .put("/table/test1")
        .header(ContentType::JSON)
        .body(SIMPLE_TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client.get("/table/all").dispatch();

    assert_eq!(
        vec![
            serde_json::from_str::<Table>(SIMPLE_TEST_DATA)
                .expect("Failed to build SIMPLE_TEST_DATA"),
            serde_json::from_str::<Table>(SIMPLE_TEST_DATA)
                .expect("Failed to build SIMPLE_TEST_DATA")
        ],
        serde_json::from_str::<Vec<Table>>(&res.body_string().expect("Failed to get a Body"))
            .expect("Failed to build body")
    );
}

#[test]
fn bad_json_syntax() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body("{bad}")
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!("key must be a string at line 1 column 2", value["data"]);
}

#[test]
fn bad_json_input() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(r#"{"bad":"things"}"#)
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "unknown field `bad`, expected one of `dice`, `heading`, `results` at line 1 column 6",
        value["data"]
    );
}

#[test]
fn unsorted_single() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/single/unsorted.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "SingleOutOfOrder: 3 was found but 4 was the expected value! at line 30 column 1",
        value["data"]
    );
}

#[test]
fn duplicate_single() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/single/duplicate.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "SingleDuplicatedValue: 2 is already represented! at line 30 column 1",
        value["data"]
    );
}

#[test]
fn oob_less_single() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/single/oob_less.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "SingleOutOfBounds: 0 is out of bounds. min:1, max:6! at line 30 column 1",
        value["data"]
    );
}

#[test]
fn oob_more_single() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/single/oob_more.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "SingleOutOfBounds: 7 is out of bounds. min:1, max:6! at line 30 column 1",
        value["data"]
    );
}

#[test]
fn unsorted_range() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/range/unsorted.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "RangeOutOfOrder: 9-10 was found but the expected start was: 5! at line 26 column 1",
        value["data"]
    );
}

#[test]
fn duplicate_range() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/range/duplicate.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "RangeOutOfOrder: 7-8 was found but the expected start was: 3! at line 30 column 1",
        value["data"]
    );
}

#[test]
fn oob_less_range() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/range/oob_less.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "RangeOutOfBounds: 1-4 is out of bounds. min:3, max:12! at line 26 column 1",
        value["data"]
    );
}

#[test]
fn oob_more_range() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(include_str!("../../test_data/errors/range/oob_more.json"))
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!(
        "RangeOutOfBounds: 11-18 is out of bounds. min:3, max:12! at line 26 column 1",
        value["data"]
    );
}

#[test]
fn table_not_found() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client.get("/table/test").dispatch();

    let value = serde_json::from_str::<serde_json::Value>(
        &res.body_string().expect("Failed to get a Body"),
    )
    .expect("Failed to build body");

    assert_eq!("TableNotFound", value["error"]);
    assert_eq!("test", value["data"]);
}
