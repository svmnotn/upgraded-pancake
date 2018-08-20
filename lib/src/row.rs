use crate::{Dice, Range, Roll, Strings};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Row {
    roll: Roll,
    value: Strings,
}

impl Row {
    pub fn new(roll: Roll, value: Strings) -> Self {
        Row { roll, value }
    }

    pub fn roll(&self) -> Option<u32> {
        self.roll.single()
    }

    pub fn range(&self) -> Option<Range> {
        self.roll.range()
    }

    pub fn value(&self) -> Strings {
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
