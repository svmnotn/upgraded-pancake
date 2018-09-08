use crate::Range;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "error", content = "data")]
pub enum Error {
    /// This is not a valid Range
    // TODO make better
    InvalidRange(String),
    /// This part of the Range is not valid
    // TODO make better
    InvalidRangeSection(String, u32),
    /// This is not a valid Dice
    // TODO make better
    InvalidDice(String),
    /// This part of the Dice is not valid
    // TODO make better
    InvalidDiceSection(String, u32),
    /// Not all values possible values were occupied
    // eprintln!("Not all values used!\n\t values: {:?}", values);
    UnusedValuesInRange(Vec<u32>),
    /// Out of Bounds
    // eprintln!("Single out of bounds!");
    SingleOutOfBounds(u32, u32, u32),
    /// Out of Order
    // eprintln!("Single out of order!");
    SingleOutOfOrder(u32, u32),
    /// Inside a defined range
    // eprintln!("Single inside previous range!");
    SingleInsidePrevRange(u32, Range),
    /// Duplicated value
    // eprintln!("Single duplicate!");
    SingleDuplicatedValue(u32),
    /// Out of Bounds
    // eprintln!("Range out of bounds!");
    RangeOutOfBounds(Range, u32, u32),
    /// Start of range is under the last value
    // eprintln!("Range less than last val!");
    RangeLTLastVal(Range, u32),
    /// Out of Order
    // eprintln!("Range out of order!");
    RangeOutOfOrder(Range, Range),
    /// Inside last range!
    // eprintln!("Range inside last range!");
    RangeInsideAnother(Range, Range),
    /// Range contains past duplicates!
    // eprintln!("Range contains past dupes!\n\tvals: {:?}\n\tvalues: {:?}", vals, values);
    RangeHasDuplicates(Range, Vec<u32>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidRange(s) => write!(f, "{} is not a valid range!", s),
            Error::InvalidRangeSection(s, i) => {
                write!(f, "{} is not a valid section of a range {}", i, s)
            }
            Error::InvalidDice(s) => write!(f, "{} is not a valid dice!", s),
            Error::InvalidDiceSection(s, i) => {
                write!(f, "{} is not a valid section of a dice {}", i, s)
            }
            Error::UnusedValuesInRange(vals) => write!(
                f,
                "the following values are not used in the range: {:?}",
                vals
            ),
            Error::SingleOutOfBounds(v, min, max) => {
                write!(f, "{} is out of bounds. min:{}, max:{}", v, min, max)
            }
            Error::SingleOutOfOrder(v, last) => write!(
                f,
                "{} is out of order, {} was before, but is larger",
                v, last
            ),
            Error::SingleInsidePrevRange(v, r) => write!(f, "{} is inside of {}", v, r),
            Error::SingleDuplicatedValue(v) => write!(f, "{} is already represented", v),
            Error::RangeOutOfBounds(r, min, max) => {
                write!(f, "{} is out of bounds. min:{}, max:{}", r, min, max)
            }
            Error::RangeLTLastVal(r, last) => {
                write!(f, "the start of {} is less than the last val: {}", r, last)
            }
            Error::RangeOutOfOrder(r, r_last) => {
                write!(f, "the start of {} is less than the start of {}", r, r_last)
            }
            Error::RangeInsideAnother(r, r_last) => write!(f, "{} is inside of {}", r, r_last),
            Error::RangeHasDuplicates(r, dups) => {
                write!(f, "{} has the following duplicates: {:?}", r, dups)
            }
        }
    }
}
