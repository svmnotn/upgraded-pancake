use crate::{error::Error, Range, Result, RowValidation};
use serde_derive::{Deserialize, Serialize};
use std::{cmp::Ordering, ops::RangeInclusive};

/// Either a single value or a `Range` of values
/// that can be rolled on a `Table`
#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
#[serde(untagged)]
pub enum Roll {
    /// This roll is determined by a single value
    /// of the dice
    Single(u32),
    /// This roll is determined by a `Range` of values
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
    /// The value that this `Roll` covers
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

    /// The `Range` of values that this `Roll` covers
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

    /// Is this a valid `Roll`?
    // TODO: Change to just crate once issue #45388 is cleared
    pub(crate) fn validate(&self, valid: &mut RowValidation) -> Result<()> {
        match self {
            Roll::Single(v) => {
                // Check if the value is valid
                if valid.is_valid_val(v) == false {
                    Err(Error::single_oob(*v, valid.min(), valid.max()))
                } else if valid.contains(v) == false {
                    // Check if it has appeared before
                    Err(Error::single_dup(*v))
                } else if valid.is_next(v) == false {
                    Err(Error::single_ooo(*v, valid.expected()))
                } else {
                    // This is fine, since the values
                    // are sorted in reverse, but
                    // we step through in order
                    valid.remove_last_val();

                    Ok(())
                }
            }
            Roll::Range(r) => {
                // Check that the range is within our bounds
                if valid.is_valid_range(r) == false {
                    Err(Error::range_oob(r.clone(), valid.min(), valid.max()))
                } else if valid.is_next(r.start()) == false {
                    Err(Error::range_ooo(r.clone(), valid.expected()))
                } else {
                    // Get all the vaules that are not in our current set of values
                    // A.K.A. duplicates
                    let duplicates: Vec<u32> = (*r.start()..=*r.end())
                        .filter(|v| valid.contains(v) == false)
                        .collect();

                    // Check if we have duplicates
                    if duplicates.is_empty() == false {
                        Err(Error::range_dup(r.clone(), duplicates))
                    } else {
                        // Remove all values in our range
                        valid.remove_range(r);

                        Ok(())
                    }
                }
            }
        }
    }
}
