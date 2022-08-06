pub mod piece;
pub mod chess;
pub mod player;
pub mod moves;
pub mod error;
pub mod display;

/*

CLI-based chess for two players.

Pieces are marked "<PIECE + player>".
- e.g. "Pw" is a white player's pawn and "Qb" is a black player's queen. King is marked with '*'.

All moves allowed, incl. promotion, en passant and castling.

*/

#[cfg(test)]
mod tests {
    use crate::{
        chess::Chess,
        piece::Piece,
        player::Player,
        moves::Move,
        error::ChessError
    };

    #[test]
    fn chess_works() {
        
        let mut test_chess = Chess::new();

        let mut player_turn = [Player::Black, Player::White].repeat(15).into_iter();

        assert!(Piece::process_move(&mut test_chess, (6, 3), (4, 3)).is_err()); // opponent's piece
        assert!(Piece::process_move(&mut test_chess, (0, 2), (3, 5)).is_err()); // own bishob behind a pawn

        Piece::process_move(&mut test_chess, (1, 3), (3, 3)).unwrap(); // Pw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (6, 3), (4, 3)).unwrap(); // Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (1, 4), (2, 4)).unwrap(); // Pw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (6, 4), (4, 4)).unwrap(); // Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (0, 1), (2, 2)).unwrap(); // Kw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (7, 5), (3, 1)).unwrap(); // Bb pinning Kw
        test_chess.turn = player_turn.next().unwrap();

        assert_eq!(
            Piece::process_move(&mut test_chess, (2, 2), (4, 3)),
            Err(ChessError::InvalidMoveinCheck)
        ); // can't move Kw

        Piece::process_move(&mut test_chess, (0, 2), (1, 3)).unwrap(); // Bw unpinning Kw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (7, 6), (5, 5)).unwrap(); // Kb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (2, 2), (4, 3)).unwrap(); // Now can move Kw and eat Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (7, 3), (4, 3)).unwrap(); // Qb eat Kw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (3, 3), (4, 4)).unwrap(); // Pw eat Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (7, 4), (7, 6)).unwrap(); // Black castle
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (4, 4), (5, 5)).unwrap(); // Pw eat Kb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (6, 6), (5, 5)).unwrap(); // Pb eat Pw
        test_chess.turn = player_turn.next().unwrap();

        Piece::process_move(&mut test_chess, (0, 3), (3, 6)).unwrap(); // Check
        test_chess.turn = player_turn.next().unwrap();

        assert!(test_chess.is_check == true);
        assert!(!test_chess.moves_left.is_empty());

        Piece::process_move(&mut test_chess, (7, 2), (3, 6)).unwrap(); // Eating checking piece
        test_chess.turn = player_turn.next().unwrap();

        assert_eq!(
            Piece::process_move(&mut test_chess, (0, 4), (0, 2)),
            Err(ChessError::InvalidMoveCannotCastle)
        ); // Bp is stopping white from castling

        Piece::process_move(&mut test_chess, (0, 6), (1, 4)).unwrap(); // Kw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (6, 0), (4, 0)).unwrap(); // Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (1, 2), (3, 2)).unwrap(); // Pw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (4, 0), (3, 0)).unwrap(); // Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (3, 2), (4, 2)).unwrap(); // Pw
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (6, 1), (4, 1)).unwrap(); // Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (4, 2), (5, 1)).unwrap(); // En passant
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (7, 0), (5, 0)).unwrap(); // Rb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (5, 1), (6, 2)).unwrap(); // Pw eat Pb
        test_chess.turn = player_turn.next().unwrap();
        Piece::process_move(&mut test_chess, (5, 0), (5, 4)).unwrap(); // Rb
        test_chess.turn = player_turn.next().unwrap();

        let piece = test_chess.board[6][2].unwrap();
        assert_eq!(
            piece.pawn_move(
                &mut test_chess,
                &(6, 2),
                &(7, 2),
                &(1, 0),
                &Player::White
            ),
            Ok(Move::Promotion)
        ); // Would be a promotion

        Piece::process_move(&mut test_chess, (1, 1), (2, 1)).unwrap(); // Pw
        test_chess.turn = player_turn.next().unwrap();

        /*

           A  B  C  D  E  F  G  H
        1 |Rw|  |  |  |*w|Bw|  |Rw|
        2 |Pw|  |  |Bw|Kw|Pw|Pw|Pw|
        3 |  |Pw|  |  |Pw|  |  |  |
        4 |Pb|Bb|  |  |  |  |Bb|  |
        5 |  |  |  |Qb|  |  |  |  |
        6 |  |  |  |  |Rb|Pb|  |  |
        7 |  |  |Pw|  |  |Pb|  |Pb|
        8 |  |Kb|  |  |  |Rb|*b|  |

        */

        assert_eq!(
            test_chess.board,
            [
                [
                    Some(Piece::Rook(Player::White)),
                    None,
                    None,
                    None,
                    Some(Piece::King(Player::White)),
                    Some(Piece::Bishob(Player::White)),
                    None,
                    Some(Piece::Rook(Player::White))
                ],
                [
                    Some(Piece::Pawn(Player::White)),
                    None,
                    None,
                    Some(Piece::Bishob(Player::White)),
                    Some(Piece::Knight(Player::White)),
                    Some(Piece::Pawn(Player::White)),
                    Some(Piece::Pawn(Player::White)),
                    Some(Piece::Pawn(Player::White))
                ],
                [
                    None,
                    Some(Piece::Pawn(Player::White)),
                    None,
                    None,
                    Some(Piece::Pawn(Player::White)),
                    None,
                    None,
                    None
                ],
                [
                    Some(Piece::Pawn(Player::Black)),
                    Some(Piece::Bishob(Player::Black)),
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::Bishob(Player::Black)),
                    None
                ],
                [
                    None,
                    None,
                    None,
                    Some(Piece::Queen(Player::Black)),
                    None,
                    None,
                    None,
                    None
                ],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::Rook(Player::Black)),
                    Some(Piece::Pawn(Player::Black)),
                    None,
                    None
                ],
                [
                    None,
                    None,
                    Some(Piece::Pawn(Player::White)),
                    None,
                    None,
                    Some(Piece::Pawn(Player::Black)),
                    None,
                    Some(Piece::Pawn(Player::Black))
                ],
                [
                    None,
                    Some(Piece::Knight(Player::Black)),
                    None,
                    None,
                    None,
                    Some(Piece::Rook(Player::Black)),
                    Some(Piece::King(Player::Black)),
                    None
                ]
            ]
        );

        Piece::process_move(&mut test_chess, (4, 3), (1, 3)).unwrap(); // Black checkmate

        assert!(test_chess.is_check == true);
        assert!(test_chess.moves_left.is_empty());

    }
}