#[derive(PartialEq)]
#[derive(Copy, Clone)]

///
/// A CellType represents the state of a cell on a Tetris Grid, which can be either empty,
/// or filled with a block.
///
/// The number provided in the block works as an identifier, because different tetrimino blocks
/// might have different colors, outlines, etc. This allows for an arbitrary number of
/// tetrominoes while also only needing two states to check.
pub enum CellType {
    Empty,
    Block(usize)
}