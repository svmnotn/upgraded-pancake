#![feature(rust_2018_preview)]

#[macro_use]
extern crate serde_derive;

pub const RNG_MIN_COL_SIZE: usize = 1;
pub const RNG_MAX_COL_SIZE: usize = 4;
pub const RNG_MIN_ROW_SIZE: usize = 2;
pub const RNG_MAX_ROW_SIZE: usize = 10;
pub const RNG_MAX_DICE_AMOUNT: u16 = 10;
pub const RNG_DICE_SIZES: [u16; 8] = [4, 6, 8, 10, 12, 20, 100, 1000];

mod table;
pub use self::table::*;
