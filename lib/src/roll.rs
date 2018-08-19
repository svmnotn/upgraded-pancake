use crate::{Dice, Range};
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
#[serde(untagged)]
pub enum Roll {
    Single(u32),
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

    pub fn valid(
        &self,
        dice: Dice,
        values: &mut Vec<u32>,
        range: &mut Range,
        val: &mut u32,
    ) -> bool {
        let lowest = dice.min();
        let highest = dice.max();

        match self {
            Roll::Single(v) => {
                if *v < lowest || *v > highest {
                    // Out of Bounds
                    eprintln!("Single out of bounds!");
                    return false;
                }

                if *v < *val {
                    // Out of Order
                    eprintln!("Single out of order!");
                    return false;
                }

                if range.contains(v) {
                    // Inside a defined range
                    eprintln!("Single inside range!");
                    return false;
                }

                if values.contains(&v) == false {
                    // Duplicated value
                    eprintln!("Single duplicate!");
                    return false;
                }

                values.retain(|x| *x != *v);

                *val = *v;
            }
            Roll::Range(r) => {
                if *r.start() < lowest || *r.end() > highest {
                    // Out of Bounds
                    eprintln!("Range out of bounds!");
                    return false;
                }
                if *r.start() < *val {
                    // Start of range is under the last value
                    eprintln!("Range less than last val!");
                    return false;
                }

                if *r.start() < *range.start() {
                    // Out of Order
                    eprintln!("Range out of order!");
                    return false;
                }

                if range.contains(r.start()) || range.contains(r.end()) {
                    // Inside last range!
                    eprintln!("Range inside last range!");
                    return false;
                }

                let vals: Vec<u32> = (*r.start()..=*r.end())
                    .filter(|v| values.contains(&v) == false)
                    .collect();

                if vals.is_empty() == false {
                    // Range contains past duplicates!
                    eprintln!(
                        "Range contains past dupes!\n\tvals: {:?}\n\tvalues: {:?}",
                        vals, values
                    );
                    return false;
                }

                // TODO check to see if there are more checks that need be done

                // TODO redo this bit, the value should be the end of the
                // range
                *range = r.clone();
                *val = r.end() + 1;
            }
        }

        true
    }
}
