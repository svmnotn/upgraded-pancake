use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleRow {
    pub roll: i64,
    pub value: String,
}

impl Distribution<SimpleRow> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SimpleRow {
        SimpleRow {
            roll: rng.gen_range(1, 101),
            value: format!("Data-{}", rng.gen_range(1, 501)),
        }
    }
}
