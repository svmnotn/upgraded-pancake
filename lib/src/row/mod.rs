use crate::error::Error::UnusedValuesInRange;
use crate::{Dice, Result};
use std::ops::Deref;

mod row;
pub use self::row::Row;

mod validation;
pub use self::validation::Validation as RowValidation;

/// A collection of `Row`s inside a `Table`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rows(#[doc(hidden)] Vec<Row>);

impl Rows {
    pub fn new(mut rows: Vec<Row>) -> Self {
        rows.sort_unstable();
        Rows(rows)
    }

    pub(crate) fn validate(&self, dice: &Dice) -> Result<()> {
        let mut val = RowValidation::new(dice);

        for row in &self.0 {
            row.roll.validate(&mut val)?;
        }

        if val.vals.is_empty() {
            Ok(())
        } else {
            Err(UnusedValuesInRange(val.vals))
        }
    }
}

impl Deref for Rows {
    type Target = Vec<Row>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
