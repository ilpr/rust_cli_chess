/*

Implementing displays for Chess, Piece and Player

*/

use std::fmt;
use crate::{
    chess::Chess,
    piece::Piece,
    player::Player
};

impl fmt::Display for Chess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut contents = String::from("   A  B  C  D  E  F  G  H\n");

        let mut count: u8 = 0;

        let rest: String = self.board
            .iter()
            .flat_map(|x| x.iter().enumerate() )
            .map(|(i, y)| {
                    
                let mut s = String::new();

                let parse = |o: &Option<Piece>| match o {
                    Some(piece) => format!("|{}", piece),
                    None => String::from("|  ")
                };
                    
                match i {
                    0 => {
                        count += 1;
                        s.push_str(&format!("{} {}", count, parse(y)));
                    },
                    7 => match count {
                        8 => s.push_str(&format!("{}|", parse(y))),
                        _ => s.push_str(&format!("{}|\n", parse(y)))
                    },
                    _ => s.push_str(&parse(y))
                }
                s
            })
            .collect()
        ;

        contents.push_str(&rest);

        write!(f, "{}", contents)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(Player::White) => write!(f, "Pw"),
            Piece::Bishob(Player::White) => write!(f, "Bw"),
            Piece::Knight(Player::White) => write!(f, "Kw"),
            Piece::Rook(Player::White) => write!(f, "Rw"),
            Piece::Queen(Player::White) => write!(f, "Qw"),
            Piece::King(Player::White) => write!(f, "*w"),
            Piece::Pawn(Player::Black) => write!(f, "Pb"),
            Piece::Bishob(Player::Black) => write!(f, "Bb"),
            Piece::Knight(Player::Black) => write!(f, "Kb"),
            Piece::Rook(Player::Black) => write!(f, "Rb"),
            Piece::Queen(Player::Black) => write!(f, "Qb"),
            Piece::King(Player::Black) => write!(f, "*b")
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::White => write!(f, "White"),
            Player::Black => write!(f, "Black"),
        }
    }
}