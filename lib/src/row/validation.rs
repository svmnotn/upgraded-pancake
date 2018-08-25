use crate::Range;

pub struct Validation {
    /// available values
    pub vals: Vec<u32>,
    /// last recorded range
    pub range: Range,
    /// last recorded value
    pub val: u32,
    pub(super) max: u32,
    pub(super) min: u32,
}

impl Validation {
    /// maximum value possible
    pub fn max(&self) -> u32 {
        self.max
    }
    /// minimum value possible
    pub fn min(&self) -> u32 {
        self.min
    }
}
