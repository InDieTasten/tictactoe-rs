use colored::Colorize;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Position {
    Empty,
    X,
    O,
}

enum GameResult {
    WinX,
    WinO,
    Tie,
}

impl Position {
    fn to_string(&self) -> String {
        match self {
            Position::Empty => " ".white().to_string(),
            Position::X => "X".red().to_string(),
            Position::O => "O".blue().to_string(),
        }
    }
}

fn main() {
    let mut board = vec![
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
    ];

    let mut current_player = Position::X;
    let result = loop {
        let input_index: usize = loop {
            print_board(&board);
            println!(
                "Player {}: What's your next position?",
                current_player.to_string()
            );
            let position_input = read_line().unwrap();
            let input_index = parse_position_index_from_literal(position_input);
            if let Some(index) = input_index {
                match board[index] {
                    Position::Empty => break index,
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

        board[input_index] = current_player.clone();

        // todo: check win/tie conditions for current player

        if !board.contains(&Position::Empty) {
            break GameResult::Tie;
        }

        // toggle current player between X and O
        current_player = match current_player {
            Position::Empty => Position::Empty,
            Position::X => Position::O,
            Position::O => Position::X,
        }
    };

    match result {
        GameResult::Tie => {
            println!("The game ended in a tie. Well played from both sides!");
        }
        GameResult::WinX => todo!(),
        GameResult::WinO => todo!(),
    };

    println!("Final board position:");
    print_board(&board);
}

fn parse_position_index_from_literal(literal: String) -> Option<usize> {
    let mut column: Option<usize> = None;
    let mut row: Option<usize> = None;

    let upper_literal = literal.to_uppercase();
    column = if column == None && upper_literal.contains('A') {
        Some(0)
    } else {
        column
    };
    column = if column == None && upper_literal.contains('B') {
        Some(1)
    } else {
        column
    };
    column = if column == None && upper_literal.contains('C') {
        Some(2)
    } else {
        column
    };

    row = if row == None && upper_literal.contains('1') {
        Some(0)
    } else {
        row
    };
    row = if row == None && upper_literal.contains('2') {
        Some(1)
    } else {
        row
    };
    row = if row == None && upper_literal.contains('3') {
        Some(2)
    } else {
        row
    };

    if let Some(c) = column {
        if let Some(r) = row {
            Some(r * 3 + c)
        } else {
            None
        }
    } else {
        None
    }
}

fn print_board(board: &Vec<Position>) {
    println!("  A   B   C ");
    println!(
        "1 {} | {} | {}",
        board[0].to_string(),
        board[1].to_string(),
        board[2].to_string()
    );
    println!(" ---+---+---");
    println!(
        "2 {} | {} | {}",
        board[3].to_string(),
        board[4].to_string(),
        board[5].to_string()
    );
    println!(" ---+---+---");
    println!(
        "3 {} | {} | {}",
        board[6].to_string(),
        board[7].to_string(),
        board[8].to_string()
    );
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
