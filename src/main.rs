use clap::{command, Parser, Subcommand, ValueEnum};

use crate::{
    game::{Game, GameResult},
    player::{AiPlayer, LocalPlayer},
};

mod game;
mod player;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Play {
        #[arg(value_enum)]
        x: PlayerKind,
        #[arg(value_enum)]
        o: PlayerKind,
    },
    Connect,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PlayerKind {
    Local,
    Ai,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Play { x, o } => {
            let mut game = Game::new(
                match x {
                    PlayerKind::Local => Box::new(LocalPlayer::new()),
                    PlayerKind::Ai => Box::new(AiPlayer::new()),
                },
                match o {
                    PlayerKind::Local => Box::new(LocalPlayer::new()),
                    PlayerKind::Ai => Box::new(AiPlayer::new()),
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
        Commands::Connect => todo!(),
    };
}
