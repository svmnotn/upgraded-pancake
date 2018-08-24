use crate::Range;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "error", content = "data")]
pub enum Error {
    // Not all values possible values were occupied
    // eprintln!("Not all values used!\n\t values: {:?}", values);
    UnusedValuesInRange(Vec<u32>),
    // Out of Bounds
    // eprintln!("Single out of bounds!");
    SingleOutOfBounds(u32, u32, u32),
    // Out of Order
    // eprintln!("Single out of order!");
    SingleOutOfOrder(u32, u32),
    // Inside a defined range
    // eprintln!("Single inside previous range!");
    SingleInsidePrevRange(u32, Range),
    // Duplicated value
    // eprintln!("Single duplicate!");
    SingleDuplicatedValue(u32),
    // Out of Bounds
    // eprintln!("Range out of bounds!");
    RangeOutOfBounds(Range, u32, u32),
    // Start of range is under the last value
    // eprintln!("Range less than last val!");
    RangeLTLastVal(Range, u32),
    // Out of Order
    // eprintln!("Range out of order!");
    RangeOutOfOrder(Range, Range),
    // Inside last range!
    // eprintln!("Range inside last range!");
    RangeInsideAnother(Range, Range),
    // Range contains past duplicates!
    // eprintln!("Range contains past dupes!\n\tvals: {:?}\n\tvalues: {:?}", vals, values);
    RangeHasDuplicates(Range, Vec<u32>),
}
