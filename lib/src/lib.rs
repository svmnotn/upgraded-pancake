#![feature(range_contains)]

#[macro_use]
extern crate serde_derive;

mod column;
pub use self::column::Column;

mod dice;
pub use self::dice::Dice;

mod range;
pub use self::range::Range;

mod roll;
pub use self::roll::Roll;

mod row;
pub use self::row::Row;

mod table;
pub use self::table::{Table, TableResult};

mod utils;
use self::utils::*;
