/*

Finalising the moves depending on type of the move.

*/

use std::io;
use crate::{
    chess::Chess,
    piece::Piece,
    player::Player
};

#[derive(Debug, PartialEq)]
pub enum Move {
    Regular,
    Castle,
    EnPassant,
    Promotion
}

impl Move {
    pub fn new(&self,
        chess: &mut Chess,
        piece: &Piece,
        old: &(usize, usize),
        new: &(usize, usize)
    ) {

        match self {
            Move::Regular => {

                chess.board[new.0][new.1] = Some(piece.to_owned());
                chess.board[old.0][old.1] = None;

                chess.moves.push((old.to_owned(), new.to_owned()));
            },
            Move::Castle => {

                let (rook_old, rook_new) = match new {
                    (0, 2) | (7, 2) => (
                            (new.0, new.1 - 2),
                            (new.0, new.1 + 1)
                    ),
                    (0, 6) | (7, 6) => (
                            (new.0, new.1 + 1),
                            (new.0, new.1 - 1)
                    ),
                    _ => ((0, 0), (0, 0))
                };

                let rook = Piece::Rook(chess.turn);

                chess.board[new.0][new.1] = Some(piece.to_owned());
                chess.board[old.0][old.1] = None;

                chess.moves.push((old.to_owned(), new.to_owned()));
                
                chess.board[rook_new.0][rook_new.1] = Some(rook);
                chess.board[rook_old.0][rook_old.1] = None;

                chess.moves.push((rook_old.to_owned(), rook_new.to_owned()));
            },
            Move::EnPassant => {

                chess.board[new.0][new.1] = Some(piece.to_owned());
                chess.board[old.0][old.1] = None;

                let pawn_to_remove = match chess.turn {
                    Player::White => (new.0 - 1, new.1),
                    Player::Black => (new.0 + 1, new.1)
                };

                chess.board[pawn_to_remove.0][pawn_to_remove.1] = None;

                chess.moves.push((old.to_owned(), new.to_owned()));
            },
            Move::Promotion => {

                println!("Your pawn reached the last row.");

                loop {
            
                    println!("Choose the piece you want to change it to - 'QUEEN', 'ROOK', 'BISHOB', 'KNIGHT' - or 'NONE' if no change.");
            
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).expect("Failed to read line");
            
                    let new_piece = match s.trim().as_ref() {
                        "QUEEN" => Piece::Queen(chess.turn),
                        "ROOK" => Piece::Rook(chess.turn),
                        "BISHOB" => Piece::Bishob(chess.turn),
                        "KNIGHT" => Piece::King(chess.turn),
                        "NONE" => Piece::Pawn(chess.turn),
                        _ => {
                            println!("Invalid input, try again.");
                            continue
                        }
                    };

                    chess.board[new.0][new.1] = Some(new_piece);
                    chess.board[old.0][old.1] = None;

                    chess.moves.push((old.to_owned(), new.to_owned()));
            
                    break
                }
            }
        }
    }
}