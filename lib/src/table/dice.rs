use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use std::fmt;
use RNG_MAX_DICE_AMOUNT;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dice {
    amount: u16,
    size: u16,
}

impl Dice {
    pub fn roll(&self) -> u64 {
        (0..self.amount).fold(0, |acc, _| {
            thread_rng().gen_range(1 as u64, (self.size + 1) as u64) + acc
        })
    }

    pub fn min(&self) -> u64 {
        self.amount as u64
    }

    pub fn max(&self) -> u64 {
        (self.amount as u64) * (self.size as u64)
    }

    pub fn amount(&self) -> u16 {
        self.amount
    }

    pub fn size(&self) -> u16 {
        self.size
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.amount, self.size)
    }
}

impl Distribution<Dice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dice {
        let sizes: [u16; 8] = [4, 6, 8, 10, 12, 20, 100, 1000];

        Dice {
            amount: rng.gen_range(1, RNG_MAX_DICE_AMOUNT),
            size: *rng.choose(&sizes).expect("sizes where empty?"),
        }
    }
}
