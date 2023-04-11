use clap::Parser;

use crate::{
    game::{GameResult, Game},
    player::{AiPlayer, LocalPlayer},
};

mod game;
mod player;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = 0)]
    ai: u8,
}

fn main() {
    let args = Args::parse();

    let mut game = Game::new(
        Box::new(LocalPlayer::new()),
        if args.ai > 0 {
            Box::new(AiPlayer::new())
        } else {
            Box::new(LocalPlayer::new())
        },
    );
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
