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
                tables::put,
                tables::get,
                tables::delete,
                tables::table_name,
                tables::table_data,
                tables::roll_saved,
                tables::roll,
                tables::static_tables,
                files::get,
            ],
        ).launch();
}
