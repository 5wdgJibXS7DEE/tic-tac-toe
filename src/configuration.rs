#[rustfmt::skip]
pub static SOLUTIONS: [i32; 8] = [
    0b000_000_111, 0b000_111_000, 0b111_000_000, // rows
    0b001_001_001, 0b010_010_010, 0b100_100_100, // columns
    0b100_010_001, 0b001_010_100]; // diagonals

pub const BOARD_FULL: i32 = 0b111_111_111;
pub const BOARD_MIN_CASE: i32 = 0;
pub const BOARD_MAX_CASE: i32 = 8;
pub const BOARD_ROW_SIZE: i32 = 3;
pub const BOARD_ROWS: i32 = 3;
