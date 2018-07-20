mod complex_row;
mod die;
mod range;
mod simple_row;

pub use self::complex_row::ComplexRow;
pub use self::die::Die;
pub use self::range::Range;
pub use self::simple_row::SimpleRow;

use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Table {
    Simple {
        die: String,
        results: Vec<SimpleRow>,
    },
    Complex {
        die: Die,
        results: Vec<ComplexRow>,
    },
}

#[derive(Debug, Serialize)]
pub struct TableResult {
    roll: i64,
    value: String,
}

impl Table {
    pub fn get(&self) -> Option<TableResult> {
        let mut rng = thread_rng();

        match self {
            Table::Simple { results: v, .. } => {
                if let Some(r) = rng.choose(&v) {
                    Some(TableResult {
                        roll: r.roll,
                        value: r.value.clone(),
                    })
                } else {
                    None
                }
            }
            Table::Complex { die, results: v } => {
                let res = (0..die.amount).fold(0, |acc, _| rng.gen_range(0, die.size) + acc);
                if let Some(val) = v
                    .iter()
                    .filter(|ComplexRow { range: r, .. }| r.contains(res))
                    .next()
                {
                    Some(TableResult {
                        roll: res,
                        value: val.value.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Distribution<Table> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Table {
        if rng.gen() {
            // Simple Table
            Table::Simple {
                die: format!("{}", rng.gen::<Die>()),
                results: {
                    let mut vec: Vec<SimpleRow> = Vec::with_capacity(10);
                    for _ in 0..rng.gen_range(1, 10) {
                        vec.push(rng.gen());
                    }
                    vec
                },
            }
        } else {
            // Complex Table
            Table::Complex {
                die: rng.gen(),
                results: {
                    let mut vec: Vec<ComplexRow> = Vec::with_capacity(10);
                    for _ in 0..rng.gen_range(1, 10) {
                        vec.push(rng.gen());
                    }
                    vec
                },
            }
        }
    }
}
