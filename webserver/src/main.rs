#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rand;

pub mod tables;
use tables::*;

const SIMPLE_TABLE_SRC: &'static str = r#"
{
  "die": "1d6",
  "results": [
    {
      "roll": 1,
      "value": "DATA"
    },
    {
      "roll": 2,
      "value": "DATA1"
    },
    {
      "roll": 3,
      "value": "DATA2"
    }
  ]
}
"#;

const COMPLEX_TABLE_SRC: &'static str = r#"
{
  "die": {
    "amount": 3,
    "size": 6
  },
  "results": [
    {
      "range": "3-6",
      "value": "DATA"
    },
    {
      "range": "7-11",
      "value": "DATA1"
    },
    {
      "range": "12-18",
      "value": "DATA2"
    }
  ]
}
"#;

#[get("/see/de/simple")]
fn see_de_simple() -> String {
    format!(
        "Simple Table Deserialization Src: {}\n\
         Simple Table Deserialization Result: {:?}",
        SIMPLE_TABLE_SRC,
        serde_json::from_str::<Table>(SIMPLE_TABLE_SRC).unwrap(),
    )
}

#[get("/see/de/complex")]
fn see_de_complex() -> String {
    format!(
        "Complex Table Deserialization Src: {}\n\
         Complex Table Deserialization Result: {:?}",
        COMPLEX_TABLE_SRC,
        serde_json::from_str::<Table>(COMPLEX_TABLE_SRC).unwrap(),
    )
}

#[get("/see/ser/simple")]
fn see_ser_simple() -> String {
    format!(
        "{}",
        serde_json::to_string_pretty(&tables::Table::Simple {
            die: String::from("2d6"),
            results: vec![
                SimpleRow {
                    roll: 2,
                    value: String::from("DATA"),
                },
                SimpleRow {
                    roll: 3,
                    value: String::from("DATA1"),
                },
                SimpleRow {
                    roll: 4,
                    value: String::from("DATA2"),
                },
            ],
        }).unwrap(),
    )
}

#[get("/see/ser/complex")]
fn see_ser_complex() -> String {
    format!(
        "{}",
        serde_json::to_string_pretty(&tables::Table::Complex {
            die: Die { amount: 2, size: 6 },
            results: vec![
                ComplexRow {
                    range: Range(2..=3),
                    value: String::from("DATA"),
                },
                ComplexRow {
                    range: Range(4..=10),
                    value: String::from("DATA1"),
                },
                ComplexRow {
                    range: Range(11..=12),
                    value: String::from("DATA2"),
                },
            ],
        }).unwrap()
    )
}

#[get("/simple")]
fn get_simple() -> String {
    serde_json::from_str::<Table>(SIMPLE_TABLE_SRC)
        .unwrap()
        .get()
}

#[get("/complex")]
fn get_complex() -> String {
    serde_json::from_str::<Table>(COMPLEX_TABLE_SRC)
        .unwrap()
        .get()
}

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                see_de_simple,
                see_de_complex,
                see_ser_simple,
                see_ser_complex,
                get_simple,
                get_complex,
                files,
            ],
        )
        .launch();
}
