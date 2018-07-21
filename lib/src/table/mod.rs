mod dice;
pub use self::dice::Dice;

mod range;
pub use self::range::Range;

mod row;
pub use self::row::Row;

mod strings;
pub use self::strings::Strings;

use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::{thread_rng, Rng};
use {RNG_DATA_SIZE, RNG_MAX_LIST_SIZE, RNG_MIN_LIST_SIZE};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    dice: Dice,
    heading: Strings,
    results: Vec<Row>,
}

#[derive(Debug, Serialize)]
pub struct TableResult {
    roll: u64,
    value: Strings,
}

impl Table {
    pub fn get(&self) -> Option<TableResult> {
        let roll = self.dice.roll();

        self.results
            .iter()
            .filter(|row| row.is(roll))
            .next()
            .map(|row| TableResult {
                roll,
                value: row.value(),
            })
    }
}

impl Distribution<Table> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Table {
        let dice: Dice = rng.gen();
        Table {
            heading: Strings::Single(
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(RNG_DATA_SIZE)
                    .collect::<String>(),
            ),
            results: {
                let mut vec: Vec<Row> = Vec::with_capacity(RNG_MAX_LIST_SIZE);
                for _ in 0..rng.gen_range(RNG_MIN_LIST_SIZE, RNG_MAX_LIST_SIZE) {
                    vec.push({
                        let data = Strings::Single(
                            thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(RNG_DATA_SIZE)
                                .collect(),
                        );

                        if rng.gen() {
                            Row::Simple {
                                roll: dice.roll(),
                                value: data,
                            }
                        } else {
                            Row::Complex {
                                range: {
                                    let init = rng.gen_range(dice.min(), dice.max() - 3);
                                    let finish = rng.gen_range(init + 1, dice.max() - 1);

                                    Range(
                                        rng.gen_range(init, finish)
                                            ..=rng.gen_range(finish + 1, dice.max()),
                                    )
                                },
                                value: data,
                            }
                        }
                    });
                }
                vec
            },
            dice,
        }
    }
}
