use crate::model::cell::CellType;
use crate::model::cell::CellType::{Block, Empty};
use crate::model::board_trait::{CellExistsError, ITetrisBoard,
                                OutsideGridError, PlaceError, SpawnError};
use crate::model::board_trait::PlaceError::{CellExists, OutsideGrid};

/// Represents a game of Tetris, in which a tetromino can be placed on a grid of cells,
/// and can be moved downwards until it collides with other blocks. A row can be cleared,
/// resetting all cells in that row to be empty.
pub struct TetrisBoard {
    score:u32,
    grid:Vec<Vec<CellType>>,
    tetromino_blocks:Vec<Vec<CellType>>,
    tetromino_pos: (usize, usize),
}

impl TetrisBoard {
    /// Constructs a tetris grid using the specified width and height for the grid size.
    pub fn new(width:usize, height:usize) -> TetrisBoard {
        TetrisBoard {
            score: 0,
            grid: TetrisBoard::empty_board(width, height),
            tetromino_blocks: vec!(),
            tetromino_pos: (0, 0)
        }
    }

    pub fn default() -> TetrisBoard {
        return TetrisBoard::new(10, 20)
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

    /// Place the blocks in the tetromino onto the grid,
    /// throwing an error if it cannot be placed
    fn place_tetromino(&mut self) -> Result<(), PlaceError>{
        for t_row in 0..self.tetromino_blocks.len() {
            for t_col in 0..self.tetromino_blocks[t_row].len() {
                let row = self.tetromino_pos.0 + t_row;
                let col = self.tetromino_pos.1 + t_col;

                if row < 0 || row > self.grid.len() || col < 0 || col > self.grid[row].len() {
                    return Err(OutsideGrid(OutsideGridError(
                        String::from("{row}, {col} is outside of the grid range.")
                    )));
                } else if self.grid[row][col] != Empty {
                    return Err(CellExists(CellExistsError(
                        String::from("{row}, {col} already contains a cell")
                    )));
                }

                self.grid[row][col] = self.tetromino_blocks[t_row][t_col].clone();
            }
        }
        return Ok(());
    }
}

impl ITetrisBoard for TetrisBoard {
    fn get_score(&self) -> u32 {
        self.score
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }


    fn get_cell(&self, row: &usize, col: &usize) -> Result<&CellType, OutsideGridError> {
        if *row >= self.height() || *col >= self.width() {
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

    fn is_row_full(&self, row: &usize) -> Result<bool, OutsideGridError> {
        match self.grid.get(*row) {
            Some(row) => Ok(row.iter().all(|c| {
                match *c {
                    Empty => false,
                    Block(_) => true
                }
            })),
            None => Err(OutsideGridError(String::from("{*row} is outside of the grid range.")))
        }
    }


    fn clear_row(&mut self, row: &usize) -> Result<(), OutsideGridError> {
        return if *row > self.height() {
            Err(OutsideGridError(String::from("{*row} is outside of the grid range.")))
        } else {
            let len = self.grid[*row].len();
            self.grid.remove(*row);
            self.grid.insert(0, TetrisBoard::empty_row(len));
            Ok(())
        }
    }

    fn update(&mut self) {
        let new_pos: (usize, usize) = (self.tetromino_pos.0 + 1, self.tetromino_pos.1);
        // - move the tetromino down
        // - place the tetromino if it touches the ground
        // - clear a row if the row is full

        todo!();
    }

    fn can_spawn(&self, tetromino: &Vec<Vec<CellType>>, col: usize) -> bool {
        // if a tetromino already exists, a new one cannot be spawned.
        if !self.tetromino_blocks.is_empty() {
            return false;
        }

        // Iterates through each row of the tetromino and ensures that the corresponding grid
        // position does not contain any empty cells.
        for t_row in 0..tetromino.len() {
            // Iterates through each column of the tetromino and ensures that there is no cell
            // in that spot on the board.
            for t_col in 0..tetromino[t_row].len() {
                let col = col + t_col;
                if col > tetromino[t_row].len()
                    || (tetromino[t_row][col] != Empty && self.grid[t_row][col] != Empty) {
                    return false;
                }
            }
        }
        return true;
    }

    fn spawn(&mut self, tetromino: Vec<Vec<CellType>>, col:usize) -> Result<(), SpawnError> {
        if !self.tetromino_blocks.is_empty() {
            return Err(SpawnError::LiveTetrominoExists);
        }
        else if !self.can_spawn(&tetromino, col) {
            return Err(SpawnError::NoRoom);
        }

        self.tetromino_blocks = tetromino.clone();
        self.tetromino_pos = (0, col);

        return Ok(());
    }
}