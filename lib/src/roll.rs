use crate::error::Error::*;
use crate::{Range, Result, RowValidation};
use std::cmp::Ordering;

/// Either a single value or a range of values
/// that can be rolled on a `Table`
#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
#[serde(untagged)]
pub enum Roll {
    /// This roll is determined by a single value
    /// of the dice
    Single(u32),
    /// This roll is determined by a range of values
    /// of the dice
    Range(Range),
}

impl PartialEq for Roll {
    fn eq(&self, other: &Roll) -> bool {
        match (self, other) {
            (Roll::Single(s), Roll::Single(o)) => *s == *o,
            (Roll::Single(s), Roll::Range(o)) => *o == *s,
            (Roll::Range(s), Roll::Single(o)) => *s == *o,
            (Roll::Range(s), Roll::Range(o)) => *s == *o,
        }
    }
}

impl PartialEq<u32> for Roll {
    fn eq(&self, other: &u32) -> bool {
        match self {
            Roll::Single(s) => *s == *other,
            Roll::Range(s) => *s == *other,
        }
    }
}

impl PartialEq<Range> for Roll {
    fn eq(&self, other: &Range) -> bool {
        match self {
            Roll::Single(s) => *other == *s,
            Roll::Range(s) => *s == *other,
        }
    }
}

impl PartialOrd<u32> for Roll {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        match self {
            Roll::Single(s) => s.partial_cmp(other),
            Roll::Range(s) => s.partial_cmp(other),
        }
    }
}

impl PartialOrd<Range> for Roll {
    fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
        match self {
            Roll::Single(s) => other.partial_cmp(s).map(Ordering::reverse),
            Roll::Range(s) => s.partial_cmp(other),
        }
    }
}

impl PartialOrd for Roll {
    fn partial_cmp(&self, other: &Roll) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Roll {
    fn cmp(&self, other: &Roll) -> Ordering {
        match (self, other) {
            (Roll::Single(s), Roll::Single(o)) => s.cmp(o),
            (Roll::Single(s), Roll::Range(o)) => o
                .partial_cmp(s)
                .expect("Other Range couldn't compare to Single")
                .reverse(),
            (Roll::Range(s), Roll::Single(o)) => s
                .partial_cmp(o)
                .expect("Range couldn't compare to Other Single"),
            (Roll::Range(s), Roll::Range(o)) => s.cmp(o),
        }
    }
}

impl From<u32> for Roll {
    fn from(r: u32) -> Self {
        Roll::Single(r)
    }
}

impl From<Range> for Roll {
    fn from(r: Range) -> Self {
        Roll::Range(r)
    }
}

impl Roll {
    /// The value that his `Roll` covers
    pub fn single(&self) -> Option<u32> {
        match self {
            Roll::Single(v) => Some(*v),
            _ => None,
        }
    }

    /// Is this `Roll` a single value?
    pub fn is_single(&self) -> bool {
        match self {
            Roll::Single(_) => true,
            _ => false,
        }
    }

    /// The range of values that this
    /// `Roll` covers
    pub fn range(&self) -> Option<Range> {
        match self {
            Roll::Range(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Is this `Roll` a range of values?
    pub fn is_range(&self) -> bool {
        match self {
            Roll::Range(_) => true,
            _ => false,
        }
    }

    /// Is this a valid roll?
    // TODO: Change to just crate once issue #45388 is cleared
    pub(crate) fn validate(&self, valid: &mut RowValidation) -> Result<()> {
        match self {
            Roll::Single(v) => {
                if *v < valid.min() || *v > valid.max() {
                    return Err(SingleOutOfBounds(*v, valid.min(), valid.max()));
                }

                if *v < valid.val {
                    return Err(SingleOutOfOrder(*v, valid.val));
                }

                if valid.range.contains(v) {
                    return Err(SingleInsidePrevRange(*v, valid.range.clone()));
                }

                if valid.vals.contains(&v) == false {
                    return Err(SingleDuplicatedValue(*v));
                }

                valid.vals.retain(|x| *x != *v);

                valid.val = *v;
            }
            Roll::Range(r) => {
                if *r.start() < valid.min() || *r.end() > valid.max() {
                    return Err(RangeOutOfBounds(r.clone(), valid.min(), valid.max()));
                }

                if *r.start() < valid.val {
                    return Err(RangeLTLastVal(r.clone(), valid.val));
                }

                if *r.start() < *valid.range.start() {
                    return Err(RangeOutOfOrder(r.clone(), valid.range.clone()));
                }

                if valid.range.contains(r.start()) || valid.range.contains(r.end()) {
                    return Err(RangeInsideAnother(r.clone(), valid.range.clone()));
                }

                let vals: Vec<u32> = (*r.start()..=*r.end())
                    .filter(|v| valid.vals.contains(&v) == false)
                    .collect();

                if vals.is_empty() == false {
                    return Err(RangeHasDuplicates(r.clone(), vals));
                }

                valid.vals.retain(|x| r.contains(x) == false);

                // TODO check to see if there are more checks that need be done

                valid.range = r.clone();
                valid.val = r.end() + 1;
            }
        }

        Ok(())
    }
}
