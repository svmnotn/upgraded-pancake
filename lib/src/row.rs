use crate::{Column, Dice, Range, Roll};

/// A row on a Table
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    roll: Roll,
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
    crate fn valid(
        &self,
        dice: Dice,
        values: &mut Vec<u32>,
        range: &mut Range,
        val: &mut u32,
    ) -> bool {
        self.roll.valid(dice, values, range, val)
    }
}

impl PartialEq<u32> for Row {
    fn eq(&self, other: &u32) -> bool {
        self.roll == *other
    }
}
