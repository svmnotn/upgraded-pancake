mod table_result;
pub use self::table_result::TableResult;

#[cfg(test)]
mod tests;

use crate::{Column, Dice, Range, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    dice: Dice,
    heading: Column,
    results: Vec<Row>,
}

impl Table {
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

    pub fn roll(&self) -> Option<TableResult> {
        let roll = self.dice.roll();

        self.results
            .iter()
            .enumerate()
            .find(|(_, row)| **row == roll)
            .map(|(i, _)| TableResult::new(roll, i))
    }

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
