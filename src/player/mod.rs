use crate::game::{board::Board, piece::Piece};

pub mod ai_player;
pub mod local_player;

pub trait Player {
    fn set_piece(&mut self, piece: Piece);
    fn pick_field(&self, board: &Board) -> usize;
}
