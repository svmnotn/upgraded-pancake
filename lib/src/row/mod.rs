use crate::error::Error;
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
            Err(Error::unused_values(val.vals))
        }
    }
}

impl From<Row> for Rows {
    fn from(row: Row) -> Self {
        Rows(vec![row])
    }
}

impl From<Vec<Row>> for Rows {
    fn from(mut rows: Vec<Row>) -> Self {
        rows.sort_unstable();
        Rows(rows)
    }
}

impl Deref for Rows {
    type Target = [Row];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
