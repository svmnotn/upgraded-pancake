use crate::{Dice, Range};

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

                if range.contains(*v) {
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

                if range.contains(*r.start()) || range.contains(*r.end()) {
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
