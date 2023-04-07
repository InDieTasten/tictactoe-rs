use clap::Parser;
use colored::Colorize;
use rand::seq::SliceRandom;
use std::fmt;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = 0)]
    ai: u8,
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Free,
    Occupied(Piece),
}

#[derive(Clone, Copy, PartialEq)]
enum Piece {
    X,
    O,
}

enum GameResult {
    Win(Piece),
    Tie
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

fn main() {
    let mut board = vec![
        Field::Free,
        Field::Free,
        Field::Free,
        Field::Free,
        Field::Free,
        Field::Free,
        Field::Free,
        Field::Free,
        Field::Free,
    ];

    let args = Args::parse();

    let mut current_piece = Piece::X;
    let result = loop {
        let input_index: usize = loop {
            print_board(&board);

            let is_ai_turn = args.ai != 0 && current_piece == Piece::O;

            println!(
                "{} {}: What's your next position?",
                if is_ai_turn { "AI" } else { "Player" },
                current_piece
            );

            let input_index = if is_ai_turn {
                // ai response
                let free_board_indices = board
                    .iter()
                    .enumerate()
                    .filter(|(_, &value)| value == Field::Free)
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();

                Some(*free_board_indices.choose(&mut rand::thread_rng()).unwrap())
            } else {
                // player response
                let position_input = read_line().unwrap();
                parse_position_index_from_literal(position_input)
            };

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
        };

        board[input_index] = Field::Occupied(current_piece);

        // check for winner
        if detect_win(&board, current_piece) {
            break GameResult::Win(current_piece);
        }

        // check for tie
        if !board.contains(&Field::Free) {
            break GameResult::Tie;
        }

        // toggle current player between X and O
        current_piece = match current_piece {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    };

    match result {
        GameResult::Tie => {
            println!("The game ended in a tie. Well played from both sides!");
        }
        GameResult::Win(piece) => {
            println!("Player {} won the game! Congratulations!", piece);
        }
    };

    println!("Final board position:");
    print_board(&board);
}

fn detect_win(board: &[Field], current_player: Piece) -> bool {

    let current_player = Field::Occupied(current_player);

    // rows
    if board[0] == current_player && board[1] == current_player && board[2] == current_player {
        return true;
    };
    if board[3] == current_player && board[4] == current_player && board[5] == current_player {
        return true;
    };
    if board[6] == current_player && board[7] == current_player && board[8] == current_player {
        return true;
    };

    // columns
    if board[0] == current_player && board[3] == current_player && board[6] == current_player {
        return true;
    };
    if board[1] == current_player && board[4] == current_player && board[7] == current_player {
        return true;
    };
    if board[2] == current_player && board[5] == current_player && board[8] == current_player {
        return true;
    };

    // diagonals
    if board[0] == current_player && board[4] == current_player && board[8] == current_player {
        return true;
    };
    if board[2] == current_player && board[4] == current_player && board[6] == current_player {
        return true;
    };

    false
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

fn print_board(board: &[Field]) {
    println!("  A   B   C ");
    println!("1 {} | {} | {}", board[0], board[1], board[2]);
    println!(" ---+---+---");
    println!("2 {} | {} | {}", board[3], board[4], board[5]);
    println!(" ---+---+---");
    println!("3 {} | {} | {}", board[6], board[7], board[8]);
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            trim_newline(&mut input);
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
