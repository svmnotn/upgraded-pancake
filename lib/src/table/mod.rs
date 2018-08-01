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

use lipsum::lipsum_title;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use crate::{RNG_MAX_LIST_SIZE, RNG_MIN_LIST_SIZE};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    dice: Dice,
    heading: Strings,
    results: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct TableResult {
    roll: u32,
    value: Strings,
}

impl Table {
    pub fn get(&self) -> Option<TableResult> {
        let roll = self.dice.roll();

        self.results
            .iter()
            .find(|row| row.is(roll))
            .map(|row| TableResult {
                roll,
                value: row.value(),
            })
    }
}

fn gen_strings(columns: usize, heading: bool) -> Strings {
    if columns > 1 {
        Strings::Multiple({
            let mut vec: Vec<String> = Vec::with_capacity(columns);
            for _ in 0..columns {
                vec.push(if heading {
                    lipsum_title()
                } else {
                    lipsum_title() + " " + &lipsum_title() + " " + &lipsum_title()
                });
            }
            vec
        })
    } else {
        Strings::Single(if heading {
            lipsum_title()
        } else {
            lipsum_title() + " " + &lipsum_title() + " " + &lipsum_title()
        })
    }
}

impl Distribution<Table> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Table {
        let dice: Dice = rng.gen();
        let columns = rng.gen_range(RNG_MIN_LIST_SIZE, RNG_MAX_LIST_SIZE);

        Table {
            results: {
                let mut vec: Vec<Row> = Vec::with_capacity(RNG_MAX_LIST_SIZE);
                for _ in 0..rng.gen_range(RNG_MIN_LIST_SIZE, RNG_MAX_LIST_SIZE) {
                    vec.push(Row {
                        roll: if rng.gen() {
                            Roll::Single(dice.roll())
                        } else {
                            Roll::Range({
                                let init = rng.gen_range(dice.min(), dice.max() - 3);
                                let finish = rng.gen_range(init + 1, dice.max() - 1);

                                Range(
                                    rng.gen_range(init, finish)
                                        ..=rng.gen_range(finish + 1, dice.max()),
                                )
                            })
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
