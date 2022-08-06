/*

Implementing chess pieces and their moves.

*/

use crate::{
    chess::Chess,
    player::Player,
    moves::Move,
    error::ChessError
};

pub static DIAGONALS: [[(i8, i8); 7]; 4] = [
    [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7)],
    [(1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7)],
    [(-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7)],
    [(-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7)]
];

pub static STRAIGHTS: [[(i8, i8); 7]; 4] = [
    [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
    [(-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0)],
    [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)],
    [(0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7)]
];

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

    pub fn process_move(
        chess: &mut Chess,
        old: (usize, usize),
        new: (usize, usize)
    ) -> Result<(), ChessError> {

        if chess.is_check == true {
            if let Some(v) = chess.moves_left.get(&old) {
                if !v.contains(&new) {
                    return Err(ChessError::InvalidMoveinCheck)
                }
            } else {
                return Err(ChessError::InvalidMoveinCheck)
            }
        }

        let piece = match chess.board[old.0][old.1] {
            Some(p) => if Piece::get_pieces(&chess.turn).contains(&p) {
                p
            } else {
                return Err(ChessError::InvalidIndex)
            },
            None => return Err(ChessError::InvalidIndex)
        };

        match chess.board[new.0][new.1] {
            Some(piece) => if Piece::get_pieces(&chess.turn).contains(&piece) {
                return Err(ChessError::InvalidMove)
            },
            None => ()
        }

        for m in piece.moves() {
            if let true = (new.0 as i8, new.1 as i8) == (old.0 as i8 + m.0, old.1 as i8 + m.1) {

                let r#move = match piece {
                    Piece::Pawn(p) => match piece.pawn_move(chess, &old, &new, &m, &p) {
                        Ok(m) => m,
                        Err(e) => return Err(e)
                    },
                    Piece::King(_p) => match piece.king_move(chess, &old, &m) {
                        Ok(m) => m,
                        Err(e) => return Err(e)
                    },
                    _ => match piece.other_move(chess, &old, &m) {
                        Ok(m) => m,
                        Err(e) => return Err(e)
                    }
                };

                if let Some(checking_player) = chess.test_check(&piece, &old, &new) {
                    if checking_player != chess.turn {
                        return Err(ChessError::InvalidMoveinCheck)
                    }
                }

                r#move.new(chess, &piece, &old, &new);
        
                chess.is_check(false);
        
                if piece == Piece::King(chess.turn) || piece == Piece::Rook(chess.turn) {
        
                    chess.change_castle_state(&piece, &old);
                    chess.change_king_position(&piece, &new);
                }
        
                return Ok(())
            }
        }

        Err(ChessError::InvalidIndex)
    }

    pub fn pawn_move(
        &self,
        chess: &Chess,
        old: &(usize, usize),
        new: &(usize, usize),
        m: &(i8, i8),
        p: &Player
    ) -> Result<Move, ChessError> {

        let dest = chess.board[new.0][new.1];

        let moves = self.moves();

        if &moves[3] == m {
            if p == &Player::White && old.0 != 1 || p == &Player::Black && old.0 != 6 {
                return Err(ChessError::InvalidMove)
            }

            if chess.get_path(old, &moves[3]).is_err() || dest != None {
                return Err(ChessError::InvalidMove)
            }
        } else if moves[1..=2].contains(&m) {
            if dest == None {
                if p == &Player::White && old.0 == 4 || p == &Player::Black && old.0 == 3 {

                    let last_move = chess.moves.len() - 1;

                    if let Some((_prev_old, prev_new)) = chess.moves.get(last_move) {

                        if chess.board[prev_new.0][prev_new.1] == Some(Piece::Pawn(p.get_opponent())) {
                            if p == &Player::White && prev_new == &(new.0 - 1, new.1) || p == &Player::Black && prev_new == &(new.0 + 1, new.1) {
                                return Ok(Move::EnPassant)
                            } else {
                                return Err(ChessError::InvalidMove)
                            }
                        } else {
                            return Err(ChessError::InvalidMove)
                        }
                    } else {
                        return Err(ChessError::InvalidMove)
                    }
                } else {
                    return Err(ChessError::InvalidMove)
                }
            } if !Piece::get_pieces(&p.get_opponent()).contains(&dest.unwrap_or(Piece::King(*p))) {
                return Err(ChessError::InvalidMove)
            }
        } else {
            if dest != None {
                return Err(ChessError::InvalidMove)
            }
        }

        if p == &Player::White && new.0 == 7 || p == &Player::Black && new.0 == 0 {
           return Ok(Move::Promotion) 
        }

        Ok(Move::Regular)
    }
            

    fn king_move(
        &self,
        chess: &Chess,
        old: &(usize, usize),
        m: &(i8, i8)
    ) -> Result<Move, ChessError> {

        if self.moves()[8..=9].contains(&m) {
            if chess.castle.get(&chess.turn).unwrap().contains(&m) {

                let path_from_king_to_rook = match m {
                    (0, 2) => (0, 3),
                    (0, -2) => (0, 4),
                    _ => (0, 0)
                };

                let path_to_check = chess.get_path(old, &path_from_king_to_rook);
                
                if let Ok(path) = path_to_check {
                    for new in path {
                        if chess.test_check(self, &old, &new).is_some() {
                            return Err(ChessError::InvalidMoveCannotCastle)
                        }
                    }

                    return Ok(Move::Castle)
                } else {
                    return Err(ChessError::InvalidMoveCannotCastle)
                }
            } else {
                return Err(ChessError::InvalidMoveCannotCastle)
            }
        }

        Ok(Move::Regular)
    }

    fn other_move(
        &self,
        chess: &Chess,
        old: &(usize, usize),
        m: &(i8, i8)
    ) -> Result<Move, ChessError> {

        if self != &Piece::Knight(chess.turn) {
            match chess.get_path(&old, &m) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }

        Ok(Move::Regular)
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