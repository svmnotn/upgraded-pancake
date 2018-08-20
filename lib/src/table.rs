mod table_result;
pub use self::table_result::TableResult;

#[cfg(test)]
mod tests;

use crate::{Column, Dice, Range, Row};

/// A table that can be rolled on
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    dice: Dice,
    heading: Column,
    results: Vec<Row>,
}

impl Table {
    /// Crate a new table
    pub fn new(dice: Dice, heading: Column, results: Vec<Row>) -> Option<Self> {
        let t = Table {
            dice,
            heading,
            results,
        };

        if t.is_valid() {
            Some(t)
        } else {
            None
        }
    }

    /// Perform a roll on this table
    pub fn roll(&self) -> Option<TableResult> {
        let roll = self.dice.roll();

        self.results
            .iter()
            .enumerate()
            .find(|(_, row)| **row == roll)
            .map(|(i, _)| TableResult::new(roll, i))
    }

    /// Check that the table is valid, this is used internally
    /// as well as when making sure that deserialized tables
    /// are correct
    // TODO impl deserialize so that this function becomes part of the
    // Deserialization step
    // EXPECTING results to be sorted such that the lowest value
    // is at the lowest index (for ranges what is taken is the start value)
    pub fn is_valid(&self) -> bool {
        let mut values: Vec<u32> = (self.dice.min()..=self.dice.max()).collect();
        let mut range = Range::from(0..=0);
        let mut val = 0;

        for row in &self.results {
            if row.valid(self.dice, &mut values, &mut range, &mut val) == false {
                return false;
            }
        }

        if values.is_empty() {
            true
        } else {
            eprintln!("Not all values used!\n\t values: {:?}", values);
            false
        }
    }
}
