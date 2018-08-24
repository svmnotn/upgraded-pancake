use crate::{Column, Dice, Range, Roll, Result};

/// A row on a Table
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    #[doc(hidden)]
    roll: Roll,
    #[doc(hidden)]
    value: Column,
}

impl Row {
    // Create a new row
    pub fn new(roll: Roll, value: Column) -> Self {
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

    /// Is this row valid?
    // TODO: Change to just crate once issue #45388 is cleared
    pub(crate) fn valid(
        &self,
        dice: Dice,
        values: &mut Vec<u32>,
        range: &mut Range,
        val: &mut u32,
    ) -> Result<()> {
        self.roll.validate(dice, values, range, val)
    }
}

impl PartialEq<u32> for Row {
    fn eq(&self, other: &u32) -> bool {
        self.roll == *other
    }
}
