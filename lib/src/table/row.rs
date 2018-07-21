use super::{Range, Strings};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Row {
    Simple { roll: u64, value: Strings },
    Complex { range: Range, value: Strings },
}

impl Row {
    pub fn is(&self, val: u64) -> bool {
        match self {
            Row::Simple { roll: r, .. } => val == *r,
            Row::Complex { range: r, .. } => r.contains(val),
        }
    }

    pub fn roll(&self) -> Option<u64> {
        if let Row::Simple { roll: r, .. } = self {
            Some(*r)
        } else {
            None
        }
    }

    pub fn range(&self) -> Option<Range> {
        if let Row::Complex { range: r, .. } = self {
            Some(r.clone())
        } else {
            None
        }
    }

    pub fn value(&self) -> Strings {
        match self {
            Row::Simple { value: v, .. } => v.clone(),
            Row::Complex { value: v, .. } => v.clone(),
        }
    }
}
