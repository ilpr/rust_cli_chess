use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ChessError {
    InvalidInput,
    InvalidIndex,
    InvalidMove,
    IncorrectPiece,
    InvalidMoveCannotCastle,
    InvalidMoveinCheck,
    UnknownError
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChessError::InvalidInput => write!(f, "Incorrect input."),
            ChessError::InvalidIndex => write!(f, "Incorrect index."),
            ChessError::InvalidMove => write!(f, "Can't move there."),
            ChessError::IncorrectPiece => write!(f, "Can't move that piece."),
            ChessError::InvalidMoveCannotCastle => write!(f, "Can't do a castle."),
            ChessError::InvalidMoveinCheck => write!(f, "Can't move that piece, you are in a check."),
            ChessError::UnknownError => write!(f, "Something went wrong.")
        }
    }
}