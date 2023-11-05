use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CellExistsError(pub usize, pub usize);

impl Display for CellExistsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pos = (self.0, self.1);
        write!(f, "Block cell already exists at position ({}, {})",
               pos.0, pos.1)
    }
}

impl Error for CellExistsError {}