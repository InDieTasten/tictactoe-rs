use std::io;

enum Field {
    Empty,
    PlayerX,
    PlayerO,
}

impl Field {
    fn get_char_representation(&self) -> &str {
        match self {
            Field::Empty => " ",
            Field::PlayerX => "X",
            Field::PlayerO => "O",
        }
    }
}

fn main() {
    let board = vec![
        Field::Empty,
        Field::PlayerX,
        Field::PlayerO,
        Field::Empty,
        Field::Empty,
        Field::Empty,
        Field::Empty,
        Field::Empty,
        Field::Empty,
    ];

    print_board(&board);

    println!("Pick your position:");
    read_line().unwrap();
}

fn print_board(board: &Vec<Field>) {
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
