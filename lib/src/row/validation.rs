use crate::{Dice, Range};

pub struct Validation {
    /// available values
    pub vals: Vec<u32>,
    /// last recorded range
    pub range: Range,
    /// last recorded value
    pub val: u32,
    /// The maximum value for any value in the Row
    max: u32,
    /// The minimum value for any value in the Row
    min: u32,
}

impl Validation {
    /// Create a new row validation data
    pub fn new(d: &Dice) -> Self {
        Validation {
            max: d.max(),
            min: d.min(),
            vals: (d.min()..=d.max()).collect(),
            range: Range::from(0..=0),
            val: 0,
        }
    }

    /// maximum value possible
    pub fn max(&self) -> u32 {
        self.max
    }

    /// minimum value possible
    pub fn min(&self) -> u32 {
        self.min
    }
}
