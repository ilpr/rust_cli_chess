pub mod chess;
pub mod r#move;
pub mod piece;
pub mod player;
pub mod constant;
pub mod display;
pub mod error;

/*

CLI-based chess for two players.

Pieces are marked "<PIECE + player>".
- e.g. "Pw" is a white player's pawn and "Qb" is a black player's queen. King is marked with '*'.

All moves allowed, incl. promotion, en passant and castling.

*/

#[cfg(test)]
mod tests {

    use crate::{
        chess::{
            Chess,
            ChessState
        },
        piece::Piece,
        player::Player,
        r#move::Move,
        error::ChessError
    };

    #[test]
    fn it_works() {
        
        let mut test_chess = Chess::new();

        let mut player_turn = [Player::Black, Player::White].repeat(15).into_iter();

        let moves = [
            Move {
                piece: Piece::Pawn(Player::White),
                from: (1, 3),
                to: (3, 3)
            },
            Move {
                piece: Piece::Pawn(Player::Black),
                from: (6, 4),
                to: (4, 4)
            },
            Move {
                piece: Piece::Pawn(Player::White),
                from: (3, 3),
                to: (4, 4)
            },
            Move {
                piece: Piece::Pawn(Player::Black),
                from: (6, 3),
                to: (4, 3)
            },
            Move { // en passant
                piece: Piece::Pawn(Player::White),
                from: (4, 4),
                to: (5, 3)
            },
            Move {
                piece: Piece::Queen(Player::Black),
                from: (7, 3),
                to: (5, 3)
            },
            Move {
                piece: Piece::Queen(Player::White),
                from: (0, 3),
                to: (5, 3)
            },
            Move {
                piece: Piece::Bishob(Player::Black),
                from: (7, 5),
                to: (5, 3)
            },
            Move {
                piece: Piece::Pawn(Player::White),
                from: (1, 4),
                to: (2, 4)
            },
            Move {
                piece: Piece::Knight(Player::Black),
                from: (7, 6),
                to: (5, 5)
            },
            Move {
                piece: Piece::Pawn(Player::White),
                from: (1, 5),
                to: (3, 5)
            },
            Move { // castling
                piece: Piece::King(Player::Black),
                from: (7, 4),
                to: (7, 6)
            },
            Move {
                piece: Piece::Knight(Player::White),
                from: (0, 1),
                to: (2, 2)
            },
            Move {
                piece: Piece::Rook(Player::Black),
                from: (7, 5),
                to: (7, 4)
            },
            Move {
                piece: Piece::Knight(Player::White),
                from: (0, 6),
                to: (2, 5)
            },
            Move { // rook is pinning pawn
                piece: Piece::Bishob(Player::Black),
                from: (5, 3),
                to: (3, 5)
            },
        ];

        /*
        
            A  B  C  D  E  F  G  H
        1 |Rw|  |Bw|  |*w|Bw|  |Rw|
        2 |Pw|Pw|Pw|  |  |  |Pw|Pw|
        3 |  |  |Kw|  |Pw|Kw|  |  |
        4 |  |  |  |  |  |Bb|  |  |
        5 |  |  |  |  |  |  |  |  |
        6 |  |  |  |  |  |Kb|  |  |
        7 |Pb|Pb|Pb|  |  |Pb|Pb|Pb|
        8 |Rb|Kb|Bb|  |Rb|  |*b|  |

        */

        moves
            .iter()
            .for_each(|m| {
                assert!(m.process_move(&mut test_chess, None).is_ok());
                test_chess.turn = player_turn.next().unwrap();
            })
        ;

        let move_pinned_piece = Move {
            piece: Piece::Pawn(Player::White),
            from: (2, 4),
            to: (3, 5)
        };

        assert_eq!(
            move_pinned_piece.process_move(&mut test_chess, None),
            Err(ChessError::KingCompromised)
        );

        let moves = [
            Move {
                piece: Piece::Pawn(Player::White),
                from: (1, 1),
                to: (2, 1)
            },
            Move {
                piece: Piece::Knight(Player::Black),
                from: (5, 5),
                to: (3, 6)
            },
            Move {
                piece: Piece::Knight(Player::White),
                from: (2, 5),
                to: (4, 6)
            },
            Move {
                piece: Piece::Knight(Player::Black),
                from: (3, 6),
                to: (1, 5)
            },
            Move {
                piece: Piece::Bishob(Player::White),
                from: (0, 2),
                to: (1, 1)
            },
            Move {
                piece: Piece::Bishob(Player::Black),
                from: (7, 2),
                to: (3, 6)
            },
            Move {
                piece: Piece::Knight(Player::White),
                from: (2, 2),
                to: (4, 1)
            }
        ];

        moves
            .iter()
            .for_each(|m| {
                assert!(m.process_move(&mut test_chess, None).is_ok());
                test_chess.turn = player_turn.next().unwrap();
            })
        ;

        let check = Move {
            piece: Piece::Rook(Player::Black),
            from: (7, 4),
            to: (2, 4)
        };

        assert_eq!(
            check.process_move(&mut test_chess, None),
            Ok(ChessState::Check {
                checked_player: Player::White,
                moves_left: vec![
                    Move {
                        piece: Piece::King(Player::White),
                        from: (0, 4),
                        to: (1, 5)
                    },
                    Move {
                        piece: Piece::King(Player::White),
                        from: (0, 4),
                        to: (1, 3)
                    },
                    Move {
                        piece: Piece::Bishob(Player::White),
                        from: (0, 5),
                        to: (1, 4)
                    },
                ]
            })
        );
        test_chess.turn = player_turn.next().unwrap();

        let moves = [
                Move {
                piece: Piece::Bishob(Player::White),
                from: (0, 5),
                to: (1, 4)
            },
            Move {
                piece: Piece::Bishob(Player::Black),
                from: (3, 6),
                to: (1, 4)
            },
            Move {
                piece: Piece::Pawn(Player::White),
                from: (1, 2),
                to: (2, 2)
            },
            Move { // check
                piece: Piece::Bishob(Player::Black),
                from: (1, 4),
                to: (3, 2)
            },
            Move {
                piece: Piece::King(Player::White),
                from: (0, 4),
                to: (1, 3)
            }
        ];

        moves
            .iter()
            .for_each(|m| {
                assert!(m.process_move(&mut test_chess, None).is_ok());
                test_chess.turn = player_turn.next().unwrap();
            })
        ;

        let checkmate = Move {
            piece: Piece::Rook(Player::Black),
            from: (2, 4),
            to: (1, 4)
        };

        assert_eq!(
            checkmate.process_move(&mut test_chess, None),
            Ok(ChessState::Checkmate)
        );

        /*
        
            A  B  C  D  E  F  G  H
        1 |Rw|  |  |  |  |  |  |Rw|
        2 |Pw|Bw|  |*w|Rb|Kb|Pw|Pw|
        3 |  |Pw|Pw|  |  |  |  |  |
        4 |  |  |Bb|  |  |Bb|  |  |
        5 |  |Kw|  |  |  |  |Kw|  |
        6 |  |  |  |  |  |  |  |  |
        7 |Pb|Pb|Pb|  |  |Pb|Pb|Pb|
        8 |Rb|Kb|  |  |  |  |*b|  |

        */
    }
}