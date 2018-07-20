use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Die {
    pub amount: i64,
    pub size: i64,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}", self.amount, self.size)
    }
}

impl Distribution<Die> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Die {
        Die {
            amount: rng.gen_range(1, 501),
            size: rng.gen_range(1, 101),
        }
    }
}
