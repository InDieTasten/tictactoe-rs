use std::fmt;

use colored::Colorize;
use serde::{Serialize, Deserialize};

use super::Piece;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Field {
    Free,
    Occupied(Piece),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field::Free => " ".white().to_string(),
                Field::Occupied(piece) => piece.to_string(),
            }
        )
    }
}
