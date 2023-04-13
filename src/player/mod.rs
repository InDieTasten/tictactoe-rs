use async_trait::async_trait;

use crate::game::{board::Board, piece::Piece};

pub use ai_player::AiPlayer;
pub use local_player::LocalPlayer;
pub use remote_player::RemotePlayer;

pub mod ai_player;
pub mod local_player;
pub mod remote_player;

#[async_trait]
pub trait Player {
    fn set_piece(&mut self, piece: Piece);
    async fn pick_field(&mut self, board: &Board) -> usize;
}
