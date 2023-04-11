use std::{fmt, ops};

use colored::Colorize;

use crate::player::Player;

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
    fn new() -> Board {
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

    fn detect_win(&self, current_player: Piece) -> bool {
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

#[derive(Clone, Copy, PartialEq)]
pub enum Field {
    Free,
    Occupied(Piece),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    X,
    O,
}

pub enum GameResult {
    Win(Piece),
    Tie,
}

pub struct Game {
    pub board: Board,
    current_piece: Piece,
    player_x: Box<dyn Player>,
    player_o: Box<dyn Player>
}

impl Game {
    pub fn new(mut player_x: Box<dyn Player>, mut player_o: Box<dyn Player>) -> Game {
        player_x.set_piece(Piece::X);
        player_o.set_piece(Piece::O);
        
        Game {
            board: Board::new(),
            current_piece: Piece::X,
            player_x,
            player_o
        }
    }

    pub fn play(&mut self) -> GameResult {
        loop {
            let input_index: usize = loop {
                let current_player = match self.current_piece {
                    Piece::X => &self.player_x,
                    Piece::O => &self.player_o
                };

                let played_index = current_player.pick_field(&self.board);

                if self.board[played_index] != Field::Free {
                    eprintln!("This field is already occupied.");
                    continue;
                } else {
                    break played_index;
                }
            };

            self.board[input_index] = Field::Occupied(self.current_piece);

            // check for winner
            if self.board.detect_win(self.current_piece) {
                break GameResult::Win(self.current_piece);
            }

            // check for tie
            if !self.board.contains(&Field::Free) {
                break GameResult::Tie;
            }

            // toggle current player between X and O
            self.current_piece = match self.current_piece {
                Piece::X => Piece::O,
                Piece::O => Piece::X,
            }
        }
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
