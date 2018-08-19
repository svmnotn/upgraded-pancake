#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

mod files;
mod tables;

fn main() {
    // Do Rocket Things!
    rocket::ignite()
        .mount(
            "/",
            routes![
                files::index,
                tables::add,
                tables::get,
                tables::tables,
                tables::roll,
                tables::static_tables,
                files::get,
            ],
        ).launch();
}
