/// The result of rolling on a table.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TableResult {
    roll: u32,
    row: usize,
}

impl TableResult {
    /// Create a new Table Result, this should only be used by the
    /// `Table::roll` method
    crate fn new(roll: u32, row: usize) -> Self {
        TableResult { roll, row }
    }

    /// Get what was rolled to get this result
    pub fn roll(&self) -> u32 {
        self.roll
    }

    /// Get the index of the row that was selected
    /// by this result
    pub fn row(&self) -> usize {
        self.row
    }
}
