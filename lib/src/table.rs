use crate::{RNG_MAX_COL_SIZE, RNG_MAX_ROW_SIZE, RNG_MIN_COL_SIZE, RNG_MIN_ROW_SIZE, Dice, Strings, Row, TableResult, Range, gen_strings};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    dice: Dice,
    heading: Strings,
    results: Vec<Row>,
}

impl Table {
    pub fn get(&self) -> Option<TableResult> {
        let roll = self.dice.roll();

        self.results
            .iter()
            .enumerate()
            .find(|(_, row)| row.is(roll))
            .map(|(i, _)| TableResult {
                roll,
                row: i as u64,
            })
    }
}


impl Distribution<Table> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Table {
        let dice: Dice = rng.gen();
        let columns = rng.gen_range(RNG_MIN_COL_SIZE, RNG_MAX_COL_SIZE);

        Table {
            results: {
                let rows = rng.gen_range(RNG_MIN_ROW_SIZE, RNG_MAX_ROW_SIZE);
                let mut vec: Vec<Row> = Vec::with_capacity(rows);
                for _ in 0..rows {
                    vec.push(Row {
                        roll: if rng.gen() {
                            dice.roll().into()
                        } else {
                            let init = rng.gen_range(dice.min(), dice.max() - 3);
                            let finish = rng.gen_range(init + 1, dice.max() - 1);

                            Range::from(
                                rng.gen_range(init, finish)..=rng.gen_range(finish + 1, dice.max()),
                            ).into()
                        },
                        value: gen_strings(columns, false),
                    });
                }
                vec
            },
            heading: gen_strings(columns, true),
            dice,
        }
    }
}
