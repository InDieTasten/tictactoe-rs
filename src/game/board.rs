use std::{fmt, ops};

use super::{field::Field, piece::Piece};

pub struct Board(pub Vec<Field>);

impl ops::Deref for Board {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Board {
    pub fn new() -> Board {
        Board(vec![
            Field::Free,
            Field::Free,
            Field::Free,
            Field::Free,
            Field::Free,
            Field::Free,
            Field::Free,
            Field::Free,
            Field::Free,
        ])
    }

    pub fn detect_win(&self, current_player: Piece) -> bool {
        let current_player = Field::Occupied(current_player);

        // rows
        if self[0] == current_player && self[1] == current_player && self[2] == current_player {
            return true;
        };
        if self[3] == current_player && self[4] == current_player && self[5] == current_player {
            return true;
        };
        if self[6] == current_player && self[7] == current_player && self[8] == current_player {
            return true;
        };

        // columns
        if self[0] == current_player && self[3] == current_player && self[6] == current_player {
            return true;
        };
        if self[1] == current_player && self[4] == current_player && self[7] == current_player {
            return true;
        };
        if self[2] == current_player && self[5] == current_player && self[8] == current_player {
            return true;
        };

        // diagonals
        if self[0] == current_player && self[4] == current_player && self[8] == current_player {
            return true;
        };
        if self[2] == current_player && self[4] == current_player && self[6] == current_player {
            return true;
        };

        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  A   B   C \n1 {} | {} | {}\n ---+---+---\n2 {} | {} | {}\n ---+---+---\n3 {} | {} | {}",
        self[0],
        self[1],
        self[2],
        self[3],
        self[4],
        self[5],
        self[6],
        self[7],
        self[8]
    )
    }
}
