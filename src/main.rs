use std::io;

enum Position {
    Empty,
    X,
    O,
}

impl Position {
    fn get_char_representation(&self) -> &str {
        match self {
            Position::Empty => " ",
            Position::X => "X",
            Position::O => "O",
        }
    }
}

fn main() {
    let board = vec![
        Position::Empty,
        Position::X,
        Position::O,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
        Position::Empty,
    ];

    let mut current_player = Position::X;
    loop {
        print_board(&board);
        println!(
            "Player {}: What's your next position?",
            current_player.get_char_representation()
        );
        let position_input = read_line().unwrap();

        // toggle current player between X and O
        current_player = match current_player {
            Position::Empty => Position::Empty,
            Position::X => Position::O,
            Position::O => Position::X,
        }
    }
}

fn print_board(board: &Vec<Position>) {
    println!("  A   B   C ");
    println!(
        "1 {} | {} | {}",
        board[0].get_char_representation(),
        board[1].get_char_representation(),
        board[2].get_char_representation()
    );
    println!(" ---+---+---");
    println!(
        "2 {} | {} | {}",
        board[3].get_char_representation(),
        board[4].get_char_representation(),
        board[5].get_char_representation()
    );
    println!(" ---+---+---");
    println!(
        "3 {} | {} | {}",
        board[6].get_char_representation(),
        board[7].get_char_representation(),
        board[8].get_char_representation()
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
