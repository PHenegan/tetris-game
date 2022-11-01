use crate::model::cell::CellType;
use crate::model::cell::CellType::Empty;
use crate::model::model_trait::{OutsideGridError, SpawnError, TetrisModelTrait};

/// Represents a game of Tetris, in which a tetromino can be placed on a grid of cells,
/// and can be moved downwards until it collides with other blocks. A row can be cleared,
/// resetting all cells in that row to be empty.
pub struct TetrisBoard {
    score:u32,
    grid:Vec<Vec<CellType>>,
    tetromino:Vec<Vec<CellType>>,
}

impl TetrisBoard {
    /// Constructs a tetris grid using the specified width and height for the grid size.
    pub fn new(width:usize, height:usize) -> TetrisBoard {
        TetrisBoard {
            score: 0,
            grid: TetrisBoard::empty_board(width, height),
            tetromino: vec!()
        }
    }
    /// creates an empty grid with the specified dimensions
    fn empty_board(width:usize, height:usize) -> Vec<Vec<CellType>> {
        let mut result:Vec<Vec<CellType>> = vec!();
        for row in 0..height {
            result.push(vec!());
            for _col in 0..width {
                result[row].push(Empty);
            }
        }
        return result;
    }

    /// Creates an empty row using the specified dimension
    fn empty_row(width:usize) -> Vec<CellType> {
        let mut result = vec!();
        for _i in 0..width {
            result.push(Empty);
        }
        result
    }
}

impl TetrisModelTrait for TetrisBoard {
    fn get_score(&self) -> u32 {
        self.score
    }

    fn get_cell(&self, row: &usize, col: &usize) -> Result<&CellType, OutsideGridError> {
        if *row >= self.grid.len() || *col >= self.grid[0].len() {
            Err(OutsideGridError(String::from("{*row}, {*col} is outside of the grid range.")))
        }
        else {
            Ok(&self.grid[*row][*col])
        }
    }

    fn is_row_empty(&self, row: &usize) -> Result<bool, OutsideGridError> {
        match self.grid.get(*row) {
            Some(row) => Ok(row.iter().all(|c| *c == Empty)),
            None => Err(
                OutsideGridError(String::from("{*row} is outside of the grid range."))
            )
        }
    }

    fn clear_row(&mut self, row: &usize) -> Result<(), OutsideGridError> {
        if *row > self.grid.len() {
            return Err(OutsideGridError(String::from("{*row} is outside of the grid range.")));
        }
        else {
            let len = self.grid[*row].len();
            self.grid.remove(*row);
            self.grid.insert(0, TetrisBoard::empty_row(len));
            return Ok(());
        }
    }

    fn update(&mut self) {
        todo!()
    }

    fn can_spawn(&self, tetromino: Vec<Vec<CellType>>, col: usize) -> bool {
        // Iterates through each row of the tetromino and ensures that the corresponding grid
        // position does not contain any empty cells.
        for t_row in 0..tetromino.len() {
            // Iterates through each column of the tetromino and ensures that there is no cell
            // in that spot on the board.
            for t_col in 0..tetromino[row].len() {
                let col = col + t_col;
                if col > tetromino[row].len()
                    || (tetromino[row][col] != Empty && self.grid[t_row][col] != Empty) {
                    return false;
                }
            }
        }
        return true;
    }

    fn spawn(&mut self, tetromino: Vec<Vec<CellType>>, col:usize) -> Result<(), SpawnError> {
        if self.tetromino.is_empty() {
            return Err(SpawnError::LiveTetrominoExists);
        }
        else if !self.can_spawn(tetromino, col) {
            return Err(SpawnError::NoRoom);
        }

        todo!()
    }
}