#![feature(range_contains, custom_attribute)]

mod column;
pub use self::column::Column;

mod constants;
use self::constants::*;

mod dice;
pub use self::dice::Dice;

pub mod error;
pub use self::error::Result;

mod range;
pub use self::range::Range;

mod roll;
pub use self::roll::Roll;

mod row;
use self::row::RowValidation;
pub use self::row::{Row, Rows};

mod table;
pub use self::table::{Table, TableResult};
