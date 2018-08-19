mod table_result;
pub use self::table_result::TableResult;

#[cfg(test)]
mod tests;

use crate::{
    gen_strings, Dice, Range, Row, Strings, RNG_MAX_COL_SIZE, RNG_MAX_ROW_SIZE, RNG_MIN_COL_SIZE,
    RNG_MIN_ROW_SIZE,
};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    dice: Dice,
    heading: Strings,
    results: Vec<Row>,
}

impl Table {
    pub fn new(dice: Dice, heading: Strings, results: Vec<Row>) -> Option<Self> {
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

impl Distribution<Table> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Table {
        fn gen_results<R: Rng + ?Sized>(
            rng: &mut R,
            dice: Dice,
            columns: usize,
            rows: usize,
        ) -> Vec<Row> {
            let mut results: Vec<Row> = Vec::with_capacity(rows);
            for _ in 0..rows {
                results.push(Row::new(
                    if rng.gen() {
                        dice.roll().into()
                    } else {
                        let init = rng.gen_range(dice.min(), dice.max() - 3);
                        let finish = rng.gen_range(init + 1, dice.max() - 1);

                        Range::from(
                            rng.gen_range(init, finish)..=rng.gen_range(finish + 1, dice.max()),
                        ).into()
                    },
                    gen_strings(columns, false),
                ));
            }

            results.sort_unstable();

            results
        }

        let dice: Dice = rng.gen();
        let columns = rng.gen_range(RNG_MIN_COL_SIZE, RNG_MAX_COL_SIZE);
        let rows = rng.gen_range(RNG_MIN_ROW_SIZE, RNG_MAX_ROW_SIZE);

        let heading = gen_strings(columns, true);

        let mut t = Table::new(dice, heading.clone(), gen_results(rng, dice, columns, rows));

        while t.is_none() {
            t = Table::new(dice, heading.clone(), gen_results(rng, dice, columns, rows));
        }

        t.expect("Loop broken before a valid Table was created?")
    }
}
