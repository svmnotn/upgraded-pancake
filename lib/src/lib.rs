#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate rand;

pub const RNG_MIN_LIST_SIZE: usize = 2;
pub const RNG_MAX_LIST_SIZE: usize = 15;
pub const RNG_MAX_DICE_AMOUNT: u16 = 10;
pub const RNG_DICE_SIZES: [u16; 8] = [4, 6, 8, 10, 12, 20, 100, 1000];

mod table;
pub use self::table::*;
