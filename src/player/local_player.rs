use std::{fmt, io::stdin};

use crate::game::{Board, Field, Piece};

use super::Player;

pub struct LocalPlayer {
    piece: Option<Piece>,
}

impl LocalPlayer {
    pub fn new() -> LocalPlayer {
        LocalPlayer {
            piece: Option::None,
        }
    }

    fn read_line() -> std::io::Result<String> {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                Self::trim_newline(&mut input);
                Ok(input)
            }
            Err(e) => Err(e),
        }
    }

    fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }

    fn parse_position_index_from_literal(literal: String) -> Option<usize> {
        let mut column: Option<usize> = None;
        let mut row: Option<usize> = None;

        let upper_literal = literal.to_uppercase();
        column = if column.is_none() && upper_literal.contains('A') {
            Some(0)
        } else {
            column
        };
        column = if column.is_none() && upper_literal.contains('B') {
            Some(1)
        } else {
            column
        };
        column = if column.is_none() && upper_literal.contains('C') {
            Some(2)
        } else {
            column
        };

        row = if row.is_none() && upper_literal.contains('1') {
            Some(0)
        } else {
            row
        };
        row = if row.is_none() && upper_literal.contains('2') {
            Some(1)
        } else {
            row
        };
        row = if row.is_none() && upper_literal.contains('3') {
            Some(2)
        } else {
            row
        };

        if let Some(c) = column {
            row.map(|r| r * 3 + c)
        } else {
            None
        }
    }
}

impl Player for LocalPlayer {
    fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    fn pick_field(&self, board: &Board) -> usize {
        match self.piece {
            None => panic!("Local player wasn't initialized with a piece type."),
            Some(piece) => loop {
                println!("{}", board);
                println!("Player {}: What's your next position?", piece);
                let position_input = Self::read_line().unwrap();
                let input_index = Self::parse_position_index_from_literal(position_input);

                if let Some(index) = input_index {
                    match board[index] {
                        Field::Free => break index,
                        _ => {
                            eprintln!("This position is already occupied!");
                            continue;
                        }
                    }
                } else {
                    eprintln!("Unable to read coordinates from input!");
                    continue;
                }
            },
        }
    }
}

impl fmt::Display for LocalPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Local Player {}",
            match self.piece {
                None => String::from("<Unkown>"),
                Some(piece) => piece.to_string(),
            }
        )
    }
}
