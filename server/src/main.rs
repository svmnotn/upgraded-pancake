#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate upgraded_pancake;

mod files;
mod tables;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                files::index,
                tables::get,
                tables::post,
                tables::get_simple,
                tables::get_complex,
                files::get,
            ],
        )
        .launch();
}
