use std::{fmt, ops, io};

use colored::Colorize;
use rand::seq::SliceRandom;


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
            Field::Free
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
    Tie
}

pub struct Game {
    pub board: Board,
    is_ai_game: bool,
    current_piece: Piece
}

impl Game {
    pub fn new(ai_player: bool) -> Game {
        Game {
            board: Board::new(),
            is_ai_game: ai_player,
            current_piece: Piece::X
        }
    }

    pub fn play(&mut self) -> GameResult {
        loop {
            let input_index: usize = loop {
                println!("{}", self.board);
    
                let is_ai_turn = self.is_ai_game && self.current_piece == Piece::O;
    
                println!(
                    "{} {}: What's your next position?",
                    if is_ai_turn { "AI" } else { "Player" },
                    self.current_piece
                );
    
                let input_index = if is_ai_turn {
                    // ai response
                    let free_board_indices = self.board
                        .iter()
                        .enumerate()
                        .filter(|(_, &value)| value == Field::Free)
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>();
    
                    Some(*free_board_indices.choose(&mut rand::thread_rng()).unwrap())
                } else {
                    // player response
                    let position_input = Game::read_line().unwrap();
                    Game::parse_position_index_from_literal(position_input)
                };
    
                if let Some(index) = input_index {
                    match self.board[index] {
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
                Piece::O => Piece::X
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
    
    fn read_line() -> io::Result<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                Game::trim_newline(&mut input);
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
                Piece::O => "O".blue().to_string()
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
                Field::Occupied(piece) => piece.to_string()
            }
        )
    }
}
