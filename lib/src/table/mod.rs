mod dice;
pub use self::dice::Dice;

mod range;
pub use self::range::Range;

mod roll;
pub use self::roll::Roll;

mod row;
pub use self::row::Row;

mod strings;
pub use self::strings::Strings;

use crate::{RNG_MAX_COL_SIZE, RNG_MAX_ROW_SIZE, RNG_MIN_COL_SIZE, RNG_MIN_ROW_SIZE};
use lipsum::lipsum_title;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    dice: Dice,
    heading: Strings,
    results: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct TableResult {
    roll: u32,
    row: u64,
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

fn gen_strings(columns: usize, heading: bool) -> Strings {
    if columns > 1 {
        let mut vec: Vec<String> = Vec::with_capacity(columns);
        for _ in 0..columns {
            vec.push(if heading {
                lipsum_title()
            } else {
                lipsum_title() + " " + &lipsum_title() + " " + &lipsum_title()
            });
        }
        vec.into()
    } else if heading {
        lipsum_title().into()
    } else {
        (lipsum_title() + " " + &lipsum_title() + " " + &lipsum_title()).into()
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
