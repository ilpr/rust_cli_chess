#[derive(Debug, PartialEq)]
pub enum ChessError {
    UnableToParseInput,
    PieceBelongsToOpponent,
    NotAllowedMove,
    NotAllowedMoveInCheck,
    EmptyTile,
    InvalidDestination,
    PathIsBlocked,
    KingCompromised,
    CastlingMoveUnavailable,
    CastlingPathIsCompromised
}
