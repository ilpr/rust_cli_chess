use crate::{
    piece::Piece,
    player::Player
};

pub static INIT_BOARD: [[Option<Piece>; 8]; 8] = [
    [
        Some(Piece::Rook(Player::White)),
        Some(Piece::Knight(Player::White)),
        Some(Piece::Bishob(Player::White)),
        Some(Piece::Queen(Player::White)),
        Some(Piece::King(Player::White)),
        Some(Piece::Bishob(Player::White)),
        Some(Piece::Knight(Player::White)),
        Some(Piece::Rook(Player::White))
    ],
    [Some(Piece::Pawn(Player::White)); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Piece::Pawn(Player::Black)); 8],
    [
        Some(Piece::Rook(Player::Black)),
        Some(Piece::Knight(Player::Black)),
        Some(Piece::Bishob(Player::Black)),
        Some(Piece::Queen(Player::Black)),
        Some(Piece::King(Player::Black)),
        Some(Piece::Bishob(Player::Black)),
        Some(Piece::Knight(Player::Black)),
        Some(Piece::Rook(Player::Black))
    ]
];

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