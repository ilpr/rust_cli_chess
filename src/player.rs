use crate::piece::Piece;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Player {
    White,
    Black
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White
        }
    }

    pub fn find_player(piece: &Piece) -> Player {

        if let true = Piece::get_pieces(&Player::White).contains(piece) {
            Player::White
        } else {
            Player::Black
        }
    }
}