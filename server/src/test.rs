use crate::rocket;
use rocket::http::ContentType;
use rocket::local::Client;
use upgraded_pancake::{Table, TableResult};

const TEST_DATA: &str = include_str!("../../test_data/simple/value_single.json");

#[test]
fn can_add_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());
}

#[test]
fn can_get_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .get("/table/test")
        .dispatch();

    assert_eq!(
        serde_json::from_str::<Table>(TEST_DATA).expect("Failed to build TEST_DATA"),
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
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let _ = client
        .delete("/table/test")
        .dispatch();
}

#[test]
fn can_roll_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .get("/table/test/roll")
        .dispatch();

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
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .put("/table/test1")
        .header(ContentType::JSON)
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .get("/table/all/id")
        .dispatch();

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
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .put("/table/test1")
        .header(ContentType::JSON)
        .body(TEST_DATA)
        .dispatch();

    assert_eq!("0", res.body_string().unwrap());

    let mut res = client
        .get("/table/all")
        .dispatch();

    assert_eq!(
        vec![
            serde_json::from_str::<Table>(TEST_DATA).expect("Failed to build TEST_DATA"),
            serde_json::from_str::<Table>(TEST_DATA).expect("Failed to build TEST_DATA")
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

    let value = serde_json::from_str::<serde_json::Value>(&res.body_string().expect("Failed to get a Body"))
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

    let value = serde_json::from_str::<serde_json::Value>(&res.body_string().expect("Failed to get a Body"))
            .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
    assert_eq!("unknown field `bad`, expected one of `dice`, `heading`, `results` at line 1 column 6", value["data"]);
}

#[test]
// Temporary should panic for unimplemented Error
#[should_panic]
fn unsorted_table() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .put("/table/test")
        .header(ContentType::JSON)
        .body(r#"{
    "dice": "1d6",
    "heading": "Simple Value Single, Test Data",
    "results": [
        {
            "roll": 1,
            "value": "DATA"
        },
        {
            "roll": 3,
            "value": "DATA2"
        },
        {
            "roll": 2,
            "value": "DATA1"
        },
        {
            "roll": 4,
            "value": "DATA3"
        },
        {
            "roll": 5,
            "value": "DATA4"
        },
        {
            "roll": 6,
            "value": "DATA5"
        }
    ]
}"#)
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(&res.body_string().expect("Failed to get a Body"))
            .expect("Failed to build body");

    assert_eq!("Serde", value["error"]);
}

#[test]
fn table_not_found() {
    let client = Client::new(rocket()).unwrap();

    let mut res = client
        .get("/table/test")
        .dispatch();

    let value = serde_json::from_str::<serde_json::Value>(&res.body_string().expect("Failed to get a Body"))
            .expect("Failed to build body");

    assert_eq!("TableNotFound", value["error"]);
    assert_eq!("test", value["data"]);
}