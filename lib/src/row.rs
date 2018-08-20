use crate::{Dice, Range, Roll, Column};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    roll: Roll,
    value: Column,
}

impl Row {
    pub fn new(roll: Roll, value: Column) -> Self {
        Row { roll, value }
    }

    pub fn roll(&self) -> Roll {
        self.roll.clone()
    }

    pub fn value(&self) -> Column {
        self.value.clone()
    }

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
