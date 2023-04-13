use std::fmt;

use colored::Colorize;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Piece {
    X,
    O,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::X => "X".red().to_string(),
                Piece::O => "O".blue().to_string(),
            }
        )
    }
}
