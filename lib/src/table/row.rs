use super::{Range, Roll, Strings};

#[derive(Debug, Serialize, Deserialize)]
pub struct Row {
    pub roll: Roll,
    pub value: Strings,
}

impl Row {
    pub fn is(&self, val: u32) -> bool {
        self.roll.is(val)
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
}
