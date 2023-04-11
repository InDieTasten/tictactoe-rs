use rand::seq::SliceRandom;

use crate::game::{Board, Field, Piece};

use super::Player;

pub struct AiPlayer {
    piece: Option<Piece>,
}

impl Player for AiPlayer {
    fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece)
    }

    fn pick_field(&self, board: &Board) -> usize {
        match self.piece {
            None => panic!("AiPlayer wasn't initialized with a piece type."),
            Some(_) => {
                let free_board_indices = board
                    .iter()
                    .enumerate()
                    .filter(|(_, &value)| value == Field::Free)
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>();

                *free_board_indices.choose(&mut rand::thread_rng()).unwrap()
            }
        }
    }
}

impl AiPlayer {
    pub fn new() -> AiPlayer {
        AiPlayer {
            piece: Option::None,
        }
    }
}
