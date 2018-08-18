#[macro_use]
extern crate serde_derive;

mod table;
pub use self::table::{Table, TableResult};

mod dice;
pub use self::dice::Dice;

mod range;
pub use self::range::Range;

mod roll;
pub use self::roll::Roll;

mod row;
pub use self::row::Row;

mod strings;
pub use self::strings::Strings;

mod utils;
use self::utils::*;
