use crate::Range;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "error", content = "data")]
pub enum Error {
    /// This is not a valid `Range`
    // TODO make better
    InvalidRange(String),
    /// This part of the `Range` is not valid
    // TODO make better
    InvalidRangeSection(String, String),
    /// This is not a valid `Dice`
    // TODO make better
    InvalidDice(String),
    /// This part of the `Dice` is not valid
    // TODO make better
    InvalidDiceSection(String, String),
    /// Not all values possible values were occupied
    // eprintln!("Not all values used!\n\t values: {:?}", values);
    UnusedValuesInRange(Vec<u32>),
    /// Out of Bounds
    // eprintln!("Single out of bounds!");
    SingleOutOfBounds(u32, u32, u32),
    /// Duplicated value
    // eprintln!("Single duplicate!");
    SingleDuplicatedValue(u32),
    /// Out of Bounds
    // eprintln!("Range out of bounds!");
    RangeOutOfBounds(Range, u32, u32),
    /// Range contains past duplicates!
    // eprintln!("Range contains past dupes!\n\tvals: {:?}\n\tvalues: {:?}", vals, values);
    RangeHasDuplicates(Range, Vec<u32>),
}

impl Error {
    pub fn invalid_range<S: Into<String>>(s: S) -> Self {
        Error::InvalidRange(s.into())
    }

    pub fn invalid_range_section<S1, S2>(data: S1, section: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Error::InvalidRangeSection(data.into(), section.into())
    }

    pub fn invalid_dice<S: Into<String>>(s: S) -> Self {
        Error::InvalidDice(s.into())
    }

    pub fn invalid_dice_section<S1, S2>(data: S1, section: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Error::InvalidDiceSection(data.into(), section.into())
    }

    pub fn unused_values<V: Into<Vec<u32>>>(vals: V) -> Self {
        Error::UnusedValuesInRange(vals.into())
    }

    pub fn single_oob<T1, T2, T3>(val: T1, min: T2, max: T3) -> Self
    where
        T1: Into<u32>,
        T2: Into<u32>,
        T3: Into<u32>,
    {
        Error::SingleOutOfBounds(val.into(), min.into(), max.into())
    }

    pub fn single_dup<T: Into<u32>>(v: T) -> Self {
        Error::SingleDuplicatedValue(v.into())
    }

    pub fn range_oob<R, T1, T2>(r: R, min: T1, max: T2) -> Self
    where
        R: Into<Range>,
        T1: Into<u32>,
        T2: Into<u32>,
    {
        Error::RangeOutOfBounds(r.into(), min.into(), max.into())
    }

    pub fn range_dup<R, V>(r: R, dup: V) -> Self
    where
        R: Into<Range>,
        V: Into<Vec<u32>>,
    {
        Error::RangeHasDuplicates(r.into(), dup.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidRange(s) => write!(f, "{}: {} is not a valid range!", stringify!(InvalidRange), s),
            Error::InvalidRangeSection(s, i) => {
                write!(f, "{}: {} is not a valid section of a range {}!", stringify!(InvalidRangeSection), i, s)
            }
            Error::InvalidDice(s) => write!(f, "{}: {} is not a valid dice!", stringify!(InvalidDice), s),
            Error::InvalidDiceSection(s, i) => {
                write!(f, "{}: {} is not a valid section of a dice {}!", stringify!(InvalidDiceSection), i, s)
            }
            Error::UnusedValuesInRange(vals) => write!(
                f,
                "{}: the following values are not used in the range: {:?}!",
                stringify!(UnusedValuesInRange),
                vals
            ),
            Error::SingleOutOfBounds(v, min, max) => {
                write!(f, "{}: {} is out of bounds. min:{}, max:{}!", stringify!(SingleOutOfBounds), v, min, max)
            }
            Error::SingleDuplicatedValue(v) => write!(f, "{}: {} is already represented!", stringify!(SingleDuplicatedValue), v),
            Error::RangeOutOfBounds(r, min, max) => {
                write!(f, "{}: {} is out of bounds. min:{}, max:{}!", stringify!(RangeOutOfBounds), r, min, max)
            }
            Error::RangeHasDuplicates(r, dups) => {
                write!(f, "{}: {} has the following duplicates: {:?}!", stringify!(RangeHasDuplicates), r, dups)
            }
        }
    }
}
