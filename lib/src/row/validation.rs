use crate::{Dice, Range};

pub struct Validation {
    /// available values
    pub vals: Vec<u32>,
    /// The maximum value for any value in the Row
    max: u32,
    /// The minimum value for any value in the Row
    min: u32,
}

impl Validation {
    /// Create a new row validation data
    pub fn new(d: &Dice) -> Self {
        let mut vals: Vec<u32> = (d.min()..=d.max()).collect();
        vals.reverse();

        Validation {
            max: d.max(),
            min: d.min(),
            vals,
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

    /// Is this value within our range
    pub fn is_valid_val(&self, val: &u32) -> bool {
        self.min <= *val && *val <= self.max
    }

    pub fn expected(&self) -> usize {
        self.vals.len() - 1
    }

    /// Is this value the one we are expecting?
    pub fn is_next(&self, val: &u32) -> bool {
        self.vals[self.expected()] == *val
    }

    /// Is this range within our range
    pub fn is_valid_range(&self, r: &Range) -> bool {
        self.min <= *r.start() && *r.end() <= self.max
    }

    /// Does our set of values contain this value
    pub fn contains(&self, v: &u32) -> bool {
        self.vals.contains(v)
    }

    /// Remove the last value in our set
    pub fn remove_last_val(&mut self) {
        self.vals.pop();
    }

    /// Remove all the values in the range from our set
    pub fn remove_range(&mut self, r: &Range) {
        self.vals.retain(|x| r.contains(x) == false);
    }
}
