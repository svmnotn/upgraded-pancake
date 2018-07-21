use super::Range;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Roll {
    Single(u32),
    Range(Range),
}

impl Roll {
    pub fn is(&self, val: u32) -> bool {
        match self {
            Roll::Single(r) => val == *r,
            Roll::Range(r) => r.contains(val),
        }
    }

    pub fn single(&self) -> Option<u32> {
        if let Roll::Single(r) = self {
            Some(*r)
        } else {
            None
        }
    }

    pub fn range(&self) -> Option<Range> {
        if let Roll::Range(r) = self {
            Some(r.clone())
        } else {
            None
        }
    }
}
