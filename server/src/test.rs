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
        .header(ContentType::JSON)
        .body(TEST_DATA)
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
        .header(ContentType::JSON)
        .body(TEST_DATA)
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
        .header(ContentType::JSON)
        .body(TEST_DATA)
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
        .get("/table/all/name")
        .header(ContentType::JSON)
        .body(TEST_DATA)
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
        .header(ContentType::JSON)
        .body(TEST_DATA)
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
