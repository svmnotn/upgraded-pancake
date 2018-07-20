use super::Range;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexRow {
    pub range: Range,
    pub value: String,
}

impl Distribution<ComplexRow> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ComplexRow {
        ComplexRow {
            range: rng.gen(),
            value: format!("Data-{}", rng.gen_range(1, 501)),
        }
    }
}
