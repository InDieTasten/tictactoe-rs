use clap::Parser;
use crate::game::Game;
use crate::game::GameResult;

mod game;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = 0)]
    ai: u8,
}

fn main() {

    let args = Args::parse();

    let mut game = Game::new(args.ai > 0);
    let result = game.play();

    match result {
        GameResult::Tie => {
            println!("The game ended in a tie. Well played from both sides!");
        }
        GameResult::Win(piece) => {
            println!("Player {} won the game! Congratulations!", piece);
        }
    };

    println!("Final board position:\n{}", game.board);
}
