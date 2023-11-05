use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents the error which occurs when a live tetromino cannot be spawned
/// `NoRoom` indicates that there is no room for a tetromino to be spawned
/// `LiveTetrominoExists` indicates that the max number of live tetrominos are already in play
#[derive(Debug)]
pub enum SpawnError {
    NoRoom,
    LiveTetrominoExists
}

impl Display for SpawnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            SpawnError::NoRoom => "No room to spawn a new tetromino",
            SpawnError::LiveTetrominoExists => "tetromino already exists"
        };
        write!(f, "{msg}")
    }
}

impl Error for SpawnError {}