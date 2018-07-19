#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rand;

pub mod tables;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! Get ready to make some Tables! :D"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
