#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Player {
    White,
    Black
}

impl Player {
    pub fn get_opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }
}