#![feature(plugin, decl_macro, rust_2018_preview)]
#![plugin(rocket_codegen)]

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
                tables::get_static,
                files::get,
            ],
        ).launch();
}
