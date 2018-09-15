#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]
// TODO update docs
//! # Upgraded Pancake Server
//!
//! This is the default backend for the upgraded-pancakes library. The library was made
//! with this project as its goal, but left aside for use by others.
//!
//! ## Methods
//!
//! The following HTTP methods are supported by the server, with `<id>` being replaced
//! by the table's identifier:
//! * **PUT** `/table/<id>`: Used to add and update `Table`s.
//! For more information check [here](#adding-a-table).
//! * **GET** `/table/<id>`: Used to obtain the JSON representation of a `Table`
//! For more information check [here](#getting-a-table).
//! * **GET** `/table/<id>/roll`: Used to roll on the specifed `Table`
//! For more information check [here](#rolling-on-a-table).
//! * **DELETE** `/table/<id>`: Used to delete the specifed `Table`
//! For more information check [here](#deleting-a-table).
//! * **GET** `/table/all`: Used to retrieve all available `Table`s as JSON
//! For more information check [here](#getting-all-the-stored-tables).
//! * **GET** `/table/all/id`: Used to retrieve all available `Table` identifiers
//! For more information check [here](#getting-all-the-stored-tables-identifiers).
//! * **POST** `/table/validate`: Used to check if the included `Table` is valid
//! For more information check [here](#validating-a-table).
//!
//! The following are HTTP methods that _might_ be removed in the future
//! * **GET** `/table/static`: Used to obtain a table from a selection of premade `Table`s,
//! useful for having sane defaults
//! * **POST** `/table`: Used to roll on the included `Table`
//!
//! The following are the HTTP methods that are used so that the server can host a fully
//! functional website:
//! * **GET** `/`: Used to obtain the index file, this should be a normal html file located
//! `front-end/build/index.html` as per the writing of this documentation
//! * **GET** `/<files>`: Used to obtain all other files that are referenced from the index,
//! the default path for the files is `front-end/build` as per the writting of this documentation
//!
//! ### Adding a Table
//!
//! To add a `Table` to the storage, one must make a **HTTP PUT** method call to `/table/<id>`
//! where `id` is the desired identifier for the `Table`. _Furthermore_, the table to add
//! must be sent as JSON in the data portion of the HTTP method call.
//!
//! The server will then return `0` if the table was correctly added, and an Error if it was
//! not. Take a look at the Errors section [here](#errors).
//!
//! ### Getting a Table
//!
//! To get a `Table` from the storage, one must make a **HTTP GET** method call to `/table/<id>`
//! where `id` is the identifier for the desired `Table`.
//!
//! The server will return the `Table` as JSON, or an Error. Take a look at the Errors section
//! [here](#errors).
//!
//! ### Rolling on a Table
//!
//! To roll on a `Table` from the storage, one must make a **HTTP GET** method call to `/table/<id>/roll`
//! where `id` is the identifier for the `Table` to roll on.
//!
//! The server will return the `TableResult` as JSON or an Error. Take a look at the Errors section
//! [here](#errors).
//!
//! ### Deleting a Table
//!
//! To remove a `Table` from storage, one must make a **HTTP DELETE** method call to `/table/<id>`
//! where `id` is the identifier for the `Table` to remove.
//!
//! The server will _not_ give a response to this.
//!
//! ### Getting all the stored Tables
//!
//! To retrieve all the tables in the storage, one must make a **HTTP GET** method call to `/table/all`.
//!
//! The server will return a list containing `Table`s or Errors, depending whether the `Table` was
//! properly retrieved from the storage. Take a look at the Errors section [here](#errors), for more
//! information on what errors could be in the returned list.
//!  
//! ### Getting all the stored Tables identifiers
//!
//! To retrieve the identifers of all the stored `Table`s, one must make a **HTTP GET** method call to
//! `/table/all/id`.
//!
//! The server will return a list containing the ids of every `Table` in the storage.
//!
//! ### Validating a Table
//!
//! To validate a `Table`, one must make a **HTTP POST** method call to `/table/validate`. _Furthermore_,
//! the table to validate must be sent as JSON in the data portion of the HTTP method call.
//!
//! The server will return `0` if the table was valid, otherwise it will return the reason why it was not.
//! Take a look at the Errors section [here](#errors), for more information on what errors could be
//! returned.
//!
//! ## Errors
//!
//! The Error type is as follows:
//! ```json
//! {
//!     "error": "<Error Type>",
//!     "data": The Error's Data
//! }
//! ```
//! 
//! ### Serde Errors
//! 
//! Serde Errors are (de)serialization errors caused by either malformed JSON or a malformed `Table`.
//! 
//! A malformed `Table` would be a table that does not contain all possible rolls, or has duplicates,
//! or out of bounds rolls."A string containing the error"
//! 
//! As such their data section is a string representing what went wrong. Their Error Type is `Serde`.

#[macro_use]
extern crate serde_derive;

mod error;
mod files;
mod tables;
#[cfg(test)]
mod test;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/",
        rocket::routes![
            files::index,
            tables::put,
            tables::get,
            tables::delete,
            tables::table_ids,
            tables::all,
            tables::roll_saved,
            tables::roll,
            tables::static_tables,
            tables::validate,
            files::get,
        ],
    )
}

fn main() {
    // Do Rocket Things!
    rocket().launch();
}
