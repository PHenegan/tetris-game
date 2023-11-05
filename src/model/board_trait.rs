use crate::model::CellType;
use crate::model::errors::{OutsideGridError, SpawnError};

/// A TetrisModelState represents the board for a game of tetris, a game where tetris blocks
/// (Tetrominoes) fall from the top of a grid and are stacked until pieces can no longer be placed
pub trait ITetrisBoard {
    /// Returns the score of the game.
    fn score(&self) -> u32;

    /// Returns the length of the board (number of cells)
    fn width(&self) -> usize;

    /// Returns the height of the board (number of cells)
    fn height(&self) -> usize;

    /// Gets the cell on the board at the position indicated by the given row and column
    /// If the given row and column is out of bounds, an error will be thrown.
    /// * `row` - the row (0-indexed) of the cell being retrieved
    /// * `col` - the column (0-indexed) of the cell being retrieved
    fn cell_at(&self, row: &usize, col: &usize) -> Result<&CellType, OutsideGridError>;

    /// Checks if the given row consists of only empty cells
    /// * `row` - the row (0-indexed) of the board to check for emptiness
    fn is_row_empty(&self, row: &usize) -> Result<bool, OutsideGridError>;

    /// Checks if the given row is filled with blocks
    /// * `row` the row (0-indexed) of the board to check
    fn is_row_full(&self, row: &usize) -> Result<bool, OutsideGridError>;

    /// Removes the given row from the board by shifting every row above it down
    /// # Arguments
    /// * `row` - the row (0-indexed) of the board to clear
    fn clear_row(&mut self, row: &usize) -> Result<(), OutsideGridError>;

    /// Applies gravity to any live Tetromino(es), placing them down if there is something touching
    /// the bottom (i.e. they can't fall anymore)
    fn update(&mut self);

    /// Returns a boolean representing whether or not the given tetromino can be placed
    /// at the top of the board at the given column
    /// * `tetromino` - a 2D grid of cells representing the tetromino to be spawned
    /// * `col` - the column for the top-left corner of the tetromino at on the board (0-indexed)
    fn can_spawn(&self, tetromino: &Vec<Vec<CellType>>, col: usize) -> bool;

    /// Spawns a new live tetromino at the top of the board, returns an error if the tetromino
    /// could not be spawned (there is no room on the grid, or there is already a live tetromino
    /// * `tetromino` - the `Tetromino` pattern to be spawned on the grid
    /// * `col` - the column to place the top left cell of the tetromino at.
    fn spawn(&mut self, tetromino: Vec<Vec<CellType>>, col:usize) -> Result<(), SpawnError>;
}