use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use upgraded_pancake::Table as PancakeTable;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Table {
    #[serde(flatten)]
    t: PancakeTable,
    #[serde(default)]
    probabilities: Vec<f64>,
}

impl Table {
    pub fn fill_in(mut self) -> Self {
        if self.probabilities.len() != self.t.rows().count() {
            self.probabilities.clear();

            for row in self.t.rows() {
                let prob = self.t.probability(row.roll());
                self.probabilities.push(prob);
            }
        }

        self
    }

    pub fn probability(&self, n: u32) -> f64 {
        self.t.probability(n)
    }

    pub fn probabilities(&self) -> Vec<f64> {
        self.probabilities.clone()
    }
}

impl Deref for Table {
    type Target = PancakeTable;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
