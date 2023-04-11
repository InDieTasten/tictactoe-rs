use crate::player::Player;

pub use board::Board;
pub use field::Field;
pub use game_result::GameResult;
pub use piece::Piece;

pub mod board;
pub mod field;
pub mod game_result;
pub mod piece;

pub struct Game {
    pub board: Board,
    current_piece: Piece,
    player_x: Box<dyn Player>,
    player_o: Box<dyn Player>,
}

impl Game {
    pub fn new(mut player_x: Box<dyn Player>, mut player_o: Box<dyn Player>) -> Game {
        player_x.set_piece(Piece::X);
        player_o.set_piece(Piece::O);

        Game {
            board: Board::new(),
            current_piece: Piece::X,
            player_x,
            player_o,
        }
    }

    pub fn play(&mut self) -> GameResult {
        loop {
            let input_index: usize = loop {
                let current_player = match self.current_piece {
                    Piece::X => &self.player_x,
                    Piece::O => &self.player_o,
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
