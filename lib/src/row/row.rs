use crate::{Column, Roll};
use serde_derive::{Deserialize, Serialize};

/// A row on a `Table`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    #[doc(hidden)]
    pub(super) roll: Roll,
    #[doc(hidden)]
    value: Column,
}

impl Row {
    // Create a new row
    pub fn new<C: Into<Column>>(roll: Roll, value: C) -> Self {
        let value: Column = value.into();

        Row { roll, value }
    }

    /// The `Roll` that most be rolled for this
    /// row to be selected.
    pub fn roll(&self) -> Roll {
        self.roll.clone()
    }

    /// The text value of this row
    pub fn value(&self) -> Column {
        self.value.clone()
    }
}

impl PartialEq<u32> for Row {
    fn eq(&self, other: &u32) -> bool {
        self.roll == *other
    }
}
