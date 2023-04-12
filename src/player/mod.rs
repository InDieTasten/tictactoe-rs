use async_trait::async_trait;

use crate::game::{board::Board, piece::Piece};

pub use ai_player::AiPlayer;
pub use local_player::LocalPlayer;

pub mod ai_player;
pub mod local_player;

#[async_trait]
pub trait Player {
    fn set_piece(&mut self, piece: Piece);
    async fn pick_field(&self, board: &Board) -> usize;
}
