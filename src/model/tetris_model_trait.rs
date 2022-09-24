use crate::model::cell::Cell;

/// A TetrisModelState represents the board for a game of tetris, a game where tetris blocks
/// (Tetrominoes) fall from the top of a grid and are stacked until pieces can no longer be placed
pub trait TetrisModelTrait {

    /// Returns a boolean representing whether or not the tetris game has ended.
    fn is_game_over(&self) -> bool;

    /// Returns the score of the game.
    fn get_score(&self) -> u32;

    /// Gets the cell on the board at the position indicated by the given row and column
    /// If the given row and column is out of bounds, an error will be thrown.
    ///
    /// # Arguments
    /// * `row` - the row (0-indexed) of the cell being retrieved
    /// * `col` - the column (0-indexed) of the cell being retrieved
    fn get_cell(&self, row: &u32, col: &u32) -> Result<&Cell, OutsideGridError>;

    /// Checks if the given row consists of only empty cells
    ///
    /// # Arguments
    /// * `row` - the row (0-indexed) of the board to check for emptiness
    fn is_row_empty(&self, row: u32) -> Result<bool, OutsideGridError>;

    /// Removes the given row from the board by shifting every row above it down
    /// # Arguments
    /// * `row` - the row (0-indexed) of the board to clear
    fn clear_row(&mut self, row: u32) -> Result<(), OutsideGridError>;

    /// Applies gravity to any live Tetromino(es), placing them down if there is something touching
    /// the bottom (i.e. they can't fall anymore)
    fn update(&mut self);

    /// Spawns a new live tetromino at the top of the board, returns an error if the tetromino
    /// could not be spawned (there is no room on the grid, or there is already a live tetromino
    ///
    /// # Arguments
    /// * `tetromino` - the `Tetromino` pattern to be spawned on the grid
    fn spawn(&mut self, tetromino: ???) -> Result<(), SpawnError>;


}


/// Represents an Error describing an attempt to place or access something
/// outside the boundaries of the grid, with an error message to display
#[derive(Debug)]
pub struct OutsideGridError(String);


/// Represents the error which occurs when a live tetromino cannot be spawned
/// `NoRoom` indicates that there is no room for a tetromino to be spawned
/// `LiveTetrominoExists` indicates that the max number of live tetrominos are already in play
#[derive(Debug)]
pub enum SpawnError {
    NoRoom,
    LiveTetrominoExists
}