mod table_result;
pub use self::table_result::TableResult;

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
    pub fn new(dice: Dice, heading: Strings, results: Vec<Row>) -> Self {
        Table {
            dice,
            heading,
            results,
        }
    }

    pub fn roll(&self) -> Option<TableResult> {
        let roll = self.dice.roll();

        self.results
            .iter()
            .enumerate()
            .find(|(_, row)| row.is(roll))
            .map(|(i, _)| TableResult::new(roll, i))
    }

    // EXPECTING results to be sorted such that the lowest value
    // is at the lowest index (for ranges what is taken is the start value)
    pub fn is_valid(&self) -> bool {
        let lowest = self.dice.min();
        let highest = self.dice.max();

        let mut total_values: Vec<u32> = (lowest..=highest).collect();

        let mut last_range = Range::from(0..=0);
        let mut last_val = 0;

        for row in &self.results {
            // TODO push this into Row / Roll
            if let Some(val) = row.roll() {
                if val < lowest || val > highest {
                    // Out of Bounds
                    return false;
                }

                if val < last_val {
                    // Out of Order
                    return false;
                }

                if last_range.contains(val) {
                    // Inside a defined range
                    return false;
                }

                if total_values.contains(&val) == false {
                    // Duplicated value
                    return false;
                }

                total_values.retain(|x| *x != val);

                last_val = val;
            } else if let Some(range) = row.range() {
                if *range.start() < lowest || *range.end() > highest {
                    // Out of Bounds
                    return false;
                }

                if *range.start() < last_val {
                    // Start of range is under the last value
                    return false;
                }

                if *range.start() < *last_range.start() {
                    // Out of Order
                    return false;
                }

                if last_range.contains(*range.start()) || last_range.contains(*range.end()) {
                    // Inside another range!
                    return false;
                }

                let values: Vec<u32> = (*range.start()..=*range.end())
                    .filter(|v| total_values.contains(&v) == false)
                    .collect();

                if values.is_empty() == false {
                    // Range contains past duplicates!
                    return false;
                }

                // TODO check to see if there are more checks that need be done

                last_range = range.clone();
            }
        }

        true
    }
}

impl Distribution<Table> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Table {
        let dice: Dice = rng.gen();
        let columns = rng.gen_range(RNG_MIN_COL_SIZE, RNG_MAX_COL_SIZE);
        let rows = rng.gen_range(RNG_MIN_ROW_SIZE, RNG_MAX_ROW_SIZE);

        let mut results: Vec<Row> = Vec::with_capacity(rows);
        for _ in 0..rows {
            results.push(Row::new(
                if rng.gen() {
                    dice.roll().into()
                } else {
                    let init = rng.gen_range(dice.min(), dice.max() - 3);
                    let finish = rng.gen_range(init + 1, dice.max() - 1);

                    Range::from(rng.gen_range(init, finish)..=rng.gen_range(finish + 1, dice.max()))
                        .into()
                },
                gen_strings(columns, false),
            ));
        }

        Table::new(dice, gen_strings(columns, true), results)
    }
}
