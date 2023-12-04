pub mod sets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i64,
    pub col: i64,
}

impl Position {
    pub fn new(row: i64, col: i64) -> Self {
        Position { row, col }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Position::new(self.row - 1, self.col),
            Position::new(self.row + 1, self.col),
            Position::new(self.row, self.col - 1),
            Position::new(self.row, self.col + 1),
        ]
    }

    pub fn manhattan_distance(self, other: Self) -> u64 {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}
