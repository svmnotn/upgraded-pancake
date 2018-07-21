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
    roll: u32,
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

fn gen_strings(columns: usize) -> Strings {
    let mut thread_rng = thread_rng();

    if columns > 1 {
        Strings::Multiple({
            let mut vec: Vec<String> = Vec::with_capacity(columns);
            for _ in 0..columns {
                vec.push(
                    thread_rng
                        .sample_iter(&Alphanumeric)
                        .take(RNG_DATA_SIZE)
                        .collect::<String>(),
                );
            }
            vec
        })
    } else {
        Strings::Single(
            thread_rng
                .sample_iter(&Alphanumeric)
                .take(RNG_DATA_SIZE)
                .collect::<String>(),
        )
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
                        value: gen_strings(columns),
                    });
                }
                vec
            },
            heading: gen_strings(columns),
            dice,
        }
    }
}
