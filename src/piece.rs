use crate::{
    player::Player,
    r#move::Move,
    constant::{
        STRAIGHTS,
        DIAGONALS
    }
};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Piece {
    Pawn(Player),
    Bishob(Player),
    Knight(Player),
    Rook(Player),
    Queen(Player),
    King(Player)
}

impl Piece {
    pub fn moves(&self) -> Vec<(i8, i8)> {
        match self {
            Piece::Pawn(p) => {
                match p {
                    Player::White => vec![(1, 0), (1, 1), (1, -1), (2, 0)],
                    Player::Black => vec![(-1, 0), (-1, 1), (-1, -1), (-2, 0)]
                }
            },
            Piece::Rook(_) => STRAIGHTS
                .into_iter()
                .flat_map(|a| a.into_iter() )
                .collect()
                ,
            Piece::Knight(_) => vec![(2, 1), (2, -1), (-2, 1), (-2, -1), (1, -2), (-1, -2), (1, 2), (-1, 2)],
            Piece::Bishob(_) => DIAGONALS
                .into_iter()
                .flat_map(|a| a.into_iter() )
                .collect()
                ,
            Piece::Queen(_) => [DIAGONALS, STRAIGHTS]
                .into_iter()
                .flat_map(|c| c.into_iter() )
                .flat_map(|a| a.into_iter() )
                .collect()
                ,
            Piece::King(_) => vec![(1, 0), (0, 1), (1, 1), (-1, 0), (0, -1), (-1, -1), (-1, 1), (1, -1), (0, 2), (0, -2)]
        }
    }

    pub fn possible_moves(
        &self,
        (x, y): &(usize, usize)
    ) -> Vec<Move> {

        let moves = self.moves();

        moves
            .iter()
            .map(|m| {
                let to = (*x as i8 + m.0, *y as i8 + m.1);
                Move {
                    piece: *self,
                    from: (*x, *y),
                    to: (to.0 as usize, to.1 as usize)
                }
            })
            .filter(|m| m.is_within_board() )
            .collect()
    }

    pub fn get_pieces(p: &Player) -> [Piece; 6] {

        match p {
            Player::White => {
                [
                    Piece::Pawn(Player::White),
                    Piece::Bishob(Player::White),
                    Piece::Knight(Player::White),
                    Piece::Rook(Player::White),
                    Piece::Queen(Player::White),
                    Piece::King(Player::White)
                ]
            },
            Player::Black => {
                [
                    Piece::Pawn(Player::Black),
                    Piece::Bishob(Player::Black),
                    Piece::Knight(Player::Black),
                    Piece::Rook(Player::Black),
                    Piece::Queen(Player::Black),
                    Piece::King(Player::Black)
                ]
            }
        }
    }
}
