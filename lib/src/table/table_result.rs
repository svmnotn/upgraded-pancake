#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TableResult {
    roll: u32,
    row: usize,
}

impl TableResult {
    crate fn new(roll: u32, row: usize) -> Self {
        TableResult { roll, row }
    }

    pub fn roll(&self) -> u32 {
        self.roll
    }

    pub fn row(&self) -> usize {
        self.row
    }
}
