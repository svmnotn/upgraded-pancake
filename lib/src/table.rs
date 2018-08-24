mod table_result;
pub use self::table_result::TableResult;

#[cfg(test)]
mod tests;

use crate::{Column, Dice, Range, Row, Result};
use crate::error::Error::UnusedValuesInRange;

/// A table that can be rolled on
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    #[doc(hidden)]
    dice: Dice,
    #[doc(hidden)]
    heading: Column,
    #[doc(hidden)]
    results: Vec<Row>,
}

impl Table {
    /// Crate a new table
    pub fn new(dice: Dice, heading: Column, results: Vec<Row>) -> Result<Self> {
        let t = Table {
            dice,
            heading,
            results,
        };

        t.validate()?;

        Ok(t)
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
    pub fn validate(&self) -> Result<()> {
        let mut values: Vec<u32> = (self.dice.min()..=self.dice.max()).collect();
        let mut range = Range::from(0..=0);
        let mut val = 0;

        for row in &self.results {
            row.validate(self.dice, &mut values, &mut range, &mut val)?;
        }

        if values.is_empty() {
            Ok(())
        } else {
            Err(UnusedValuesInRange(values))
        }
    }
}
