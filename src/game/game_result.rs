use super::piece::Piece;

pub enum GameResult {
    Win(Piece),
    Tie,
}
