use std::fmt;

use colored::Colorize;

use super::piece::Piece;

#[derive(Clone, Copy, PartialEq)]
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
