mod table_result;
pub use self::table_result::TableResult;

#[cfg(test)]
mod tests;

use crate::{Column, Dice, Result, Rows};

/// A table that can be rolled on
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    #[doc(hidden)]
    dice: Dice,
    #[doc(hidden)]
    heading: Column,
    #[doc(hidden)]
    results: Rows,
}

impl Table {
    /// Crate a new table
    pub fn new(dice: Dice, heading: Column, results: Rows) -> Result<Self> {
        results.validate(&dice)?;

        Ok(Table {
            dice,
            heading,
            results,
        })
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
    pub fn validate(&self) -> Result<()> {
        self.results.validate(&self.dice)
    }
}
