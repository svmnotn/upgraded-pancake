#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

//! # Upgraded Pancake Server
//!
//! This is the default backend for the upgraded-pancakes library. The library was made
//! with this project as its goal, but left aside for use by others.
//!
//! ## Methods
//!
//! The following HTTP methods are supported by the server, with `<name>` being replaced
//! by the table's name:
//! * **PUT** `/table/<name>`: Used to add and update tables
//! * **GET** `/table/<name>`: Used to obtain the JSON representation of a table
//! * **GET** `/table/<name>/roll`: Used to roll on the specifed table
//! * **DELETE** `/table/<name>`: Used to delete the specifed table
//! * **GET** `/table/all/name`: Used to retrieve all available table names
//! * **GET** `/table/all/data`: Used to retrieve all available tables as JSON
//! * **POST** `/table/validate`: Used to make sure the included table is valid
//!
//! The following are HTTP methods that _might_ be removed in the future
//! * **GET** `/table/static`: Used to obtain a table from a selection of premade tables,
//! useful for having sane defaults
//! * **POST** `/table`: Used to roll on the included table
//!
//! The following are the HTTP methods that are used so that the server can host a fully
//! functional website:
//! * **GET** `/`: Used to obtain the index file, this should be a normal html file located
//! `front-end/build/index.html` as per the writing of this documentation
//! * **GET** `/<files>`: Used to obtain all other files that are referenced from the index,
//! the default path for the files is `front-end/build` as per the writting of this documentation
//!
//! ### Adding a table
//!
//! To add a table to the storage one must make a **HTTP PUT** method call to `/table/<name>`
//! where `name` is the desired identifier for the table. _Furthermore_, the table to add
//! must be sent as JSON in the data portion of the HTTP method call.
//!
//! The server will then return `true` if the table was correctly added, and `false` if it was
//! not. As of writting this documentation, the only way for a table to be denied is for it
//! to not be valid.

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
                tables::validate,
                files::get,
            ],
        ).launch();
}
