use std::error::Error;
use crate::model::CellType;
use crate::model::CellType::{Block, Empty};
use crate::model::ITetrisBoard;
use crate::model::errors::{CellExistsError, OutsideGridError, SpawnError};

/// Represents a game of Tetris, in which a tetromino can be placed on a grid of cells,
/// and can be moved downwards until it collides with other blocks. A row can be cleared,
/// resetting all cells in that row to be empty.
pub struct TetrisBoard {
    score: u32,
    grid: Vec<Vec<CellType>>,
    tetromino_blocks: Vec<Vec<CellType>>,
    tetromino_pos: (usize, usize), // row, col
}

impl TetrisBoard {
    /// Constructs a tetris grid using the specified width and height for the grid size.
    pub fn new(width: usize, height: usize) -> TetrisBoard {
        TetrisBoard {
            score: 0,
            grid: TetrisBoard::empty_board(width, height),
            tetromino_blocks: vec!(),
            tetromino_pos: (0, 0),
        }
    }

    pub fn default() -> TetrisBoard {
        return TetrisBoard::new(10, 20);
    }

    /// creates an empty grid with the specified dimensions
    fn empty_board(width: usize, height: usize) -> Vec<Vec<CellType>> {
        let mut result: Vec<Vec<CellType>> = vec!();
        for _row in 0..height {
            result.push(TetrisBoard::empty_row(width));
        }
        return result;
    }

    /// Creates an empty row using the specified dimension
    fn empty_row(width: usize) -> Vec<CellType> {
        let mut result = vec!();
        for _i in 0..width {
            result.push(Empty);
        }
        result
    }

    /// Are any cells in the falling tetromino block touching the a block or the ground?
    fn is_tetromino_grounded(&self) -> bool{
        for row in 0..self.tetromino_blocks.len() {
            for col in 0..self.tetromino_blocks[row].len() {
                let pos_below = (self.tetromino_pos.0 + row + 1, self.tetromino_pos.1 + col + 1);
                if self.is_cell_filled(pos_below) || self.is_cell_ground(pos_below) {
                    return true;
                }
            }
        }
        return false;
    }

    /// Does the given position contain a cell?
    fn is_cell_filled(&self, pos:(usize, usize)) -> bool {
        return (pos.0 < self.tetromino_blocks.len() && pos.1 < self.tetromino_blocks[pos.1].len())
            && self.tetromino_blocks[pos.0][pos.1] != Empty;
    }

    /// Is the given position in the bottom-most row?
    fn is_cell_ground(&self, pos:(usize, usize)) -> bool {
        return pos.0 == self.tetromino_blocks.len() - 1;
    }

    /// Place the blocks in the tetromino onto the grid,
    /// throwing an error if it cannot be placed
    fn place_tetromino(&mut self) -> Result<(), dyn Error> {
        for t_row in 0..self.tetromino_blocks.len() {
            for t_col in 0..self.tetromino_blocks[t_row].len() {
                let row = self.tetromino_pos.0 + t_row;
                let col = self.tetromino_pos.1 + t_col;

                if row > self.grid.len() || col > self.grid[row].len() {
                    return Err(OutsideGridError::OutsidePos(row, col));
                } else if self.grid[row][col] != Empty {
                    return Err(CellExistsError(row, col));
                }

                self.grid[row][col] = self.tetromino_blocks[t_row][t_col].clone();
            }
        }
        return Ok(());
    }
}

impl ITetrisBoard for TetrisBoard {
    fn score(&self) -> u32 {
        self.score
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn cell_at(&self, row: &usize, col: &usize) -> Result<&CellType, OutsideGridError> {
        // TODO: make this get from the tetromino block as well
        if *row >= self.height() || *col >= self.width() {
            Err(OutsideGridError::OutsidePos(*row, *col))
        } else {
            Ok(&self.grid[*row][*col])
        }
    }

    fn is_row_empty(&self, row: &usize) -> Result<bool, OutsideGridError> {
        match self.grid.get(*row) {
            Some(row) => Ok(row.iter().all(|c| *c == Empty)),
            None => Err(OutsideGridError::OutsideRow(*row))
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
            None => Err(OutsideGridError::OutsideRow(*row))
        }
    }


    fn clear_row(&mut self, row: &usize) -> Result<(), OutsideGridError> {
        return if *row > self.height() {
            Err(OutsideGridError::OutsideRow(*row))
        } else {
            let len = self.grid[*row].len();
            self.grid.remove(*row);
            self.grid.insert(0, TetrisBoard::empty_row(len));
            Ok(())
        };
    }

    fn update(&mut self) {
        // - place tetromino if touching the ground, otherwise move it down
        let mut err = Ok(());
        if self.is_tetromino_grounded() {
            err = self.place_tetromino();
        } else {
            self.tetromino_pos = (self.tetromino_pos.0 + 1, self.tetromino_pos.1);
        }
        // - clear a row if the row is full
        let mut lines_cleared = 0;
        for row in 0..self.tetromino_blocks.len() {
            if self.is_row_full(&row).unwrap() {
                let width = self.tetromino_blocks[row].len();
                self.tetromino_blocks.remove(row);
                self.tetromino_blocks.insert(0, TetrisBoard::empty_row(width));
                lines_cleared += 1;
            }
        }

        // update the score based on tetris score calculation
        self.score += match lines_cleared {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            5.. => 1200 * (lines_cleared - 3)
        };

        // - move the tetromino down
        self.tetromino_pos.0 += 1;
        self.tetromino_pos.1 += 1;
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

    fn spawn(&mut self, tetromino: Vec<Vec<CellType>>, col: usize) -> Result<(), SpawnError> {
        if !self.tetromino_blocks.is_empty() {
            return Err(SpawnError::LiveTetrominoExists);
        } else if !self.can_spawn(&tetromino, col) {
            return Err(SpawnError::NoRoom);
        }

        self.tetromino_blocks = tetromino.clone();
        self.tetromino_pos = (0, col);

        return Ok(());
    }
}