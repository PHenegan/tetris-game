use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents an Error describing an attempt to place or access something
/// outside the boundaries of the grid, with an error message to display
#[derive(Debug)]
pub enum OutsideGridError {
    OutsideRow(usize),
    OutsideCol(usize),
    OutsidePos(usize, usize)
}

impl Display for OutsideGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_type = match self {
            OutsideGridError::OutsidePos(row, col) => format!("Position ({row}, {col})"),
            OutsideGridError::OutsideRow(row) => format!("Row {row}"),
            OutsideGridError::OutsideCol(col) => format!("Col {col}")
        };
        write!(f, "{error_type} is outside the tetris board")
    }
}

impl Error for OutsideGridError {}