mod board_trait;
mod cell;
mod tetris_board;
mod errors;

// flatten hierarchy for easier use
pub use board_trait::ITetrisBoard;
pub use cell::CellType;
pub use tetris_board::TetrisBoard;
