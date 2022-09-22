use std::{
    io,
    collections::HashMap
};
use crate::{
    chess::{
        Chess,
        ChessState
    },
    piece::Piece,
    player::Player,
    error::ChessError
};

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Castle,
    EnPassant,
    Promotion,
    PawnEat,
    PawnTwo,
    PawnOne,
    Other
}

impl MoveType {
    pub fn determine_type(
        chess: &Chess,
        m: &Move
    ) -> Self {

        let moves = m.piece.moves();
        let dif = (
            m.to.0 as i8 - m.from.0 as i8,
            m.to.1 as i8 - m.from.1 as i8
        );

        match m.piece {
            Piece::King(_) => if moves[8..=9].contains(&dif) {
                MoveType::Castle
            } else {
                MoveType::Other
            },
            Piece::Pawn(p) => if &p == &Player::White && m.to.0 == 7 {
                MoveType::Promotion
            } else if &p == &Player::Black && m.to.0 == 0 {
                MoveType::Promotion
            } else if moves[1..=2].contains(&dif) {
                if let None = chess.board[m.to.0][m.to.1] {
                    MoveType::EnPassant
                } else {
                    MoveType::PawnEat
                }
            } else if moves[3] == dif {
                MoveType::PawnTwo
            } else {
                MoveType::PawnOne
            },
            _ => MoveType::Other
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub piece: Piece,
    pub from: (usize, usize),
    pub to: (usize, usize)
}

impl Move {

    pub fn from_input(chess: &Chess, input: String) -> Result<Self, ChessError> {

        let err = Err(ChessError::UnableToParseInput);

        match input.trim().split_whitespace().collect::<Vec<&str>>().as_slice() {
            [from, to] => match (Move::get_index_codes().get(*from), Move::get_index_codes().get(*to)) {
                (Some(from), Some(to)) => {
                    let piece = match chess.find_piece(from) {
                        Some(p) => p,
                        None => return Err(ChessError::EmptyTile)
                    };
                    Ok(Move {
                        piece,
                        from: *from,
                        to: *to
                    })
                },
                _ => err
            },
            _ => err
        }
    }

    fn get_index_codes() -> HashMap<String, (usize, usize)> {

        Move::get_codes().into_iter().zip(Move::get_indexes().into_iter()).collect()
    }

    fn get_codes() -> Vec<String> {

        let letters = ["A", "B", "C", "D", "E", "F", "G", "H"];
        let nums = ["1", "2", "3", "4", "5", "6", "7", "8"];

        letters
            .iter()
            .fold(Vec::new(), |mut acc, l| {

                nums.into_iter().for_each(|n| acc.push( l.to_string() + n ) );

                acc
            })
    }

    fn get_indexes() -> Vec<(usize, usize)> {

        (0..8)
            .fold(Vec::new(), |mut acc, y| {

                (0..8).for_each(|x| acc.push((x, y)) );

                acc
            })
    }

    pub fn is_within_board(&self) -> bool {

        let check = |(x, y): (&usize, &usize)| {
            if (0..8).contains(x) && (0..8).contains(y) {
                true
            } else {
                false
            }
        };

        [&self.from, &self.to]
            .iter()
            .all(|(x, y)| check((x, y)) == true )
    }

    pub fn process_move(
        &self,
        chess: &mut Chess,
        test: Option<&Player>
    ) -> Result<ChessState, ChessError> {

        if let Err(e) = self.regular_tests(chess, test) {
            return Err(e)
        }

        let move_type = match self.specific_tests(chess) {
            Ok(mt) => mt,
            Err(e) => return Err(e)
        };

        if let Some(_) = chess.is_check(&chess.turn.opponent(), Some(self)) {
            return Err(ChessError::KingCompromised)
        }

        if let Some(_) = test {
            return Ok(ChessState::Normal)
        }

        Ok(self.finalise_move(chess, &move_type))
    }

    pub fn is_valid_destination(
        &self,
        chess: &Chess,
        player_to_move: Player
    ) -> bool {

        if let Some(piece) = chess.board[self.to.0][self.to.1] {
            if Piece::get_pieces(&player_to_move.opponent()).contains(&piece) {
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn is_valid_move(
        &self,
        chess: &Chess,
        test: Option<&Player>
    ) -> Result<MoveType, ChessError> {

        if let Err(e) = self.regular_tests(chess, test) {
            return Err(e)
        }

        self.specific_tests(chess)
    }

    fn regular_tests(
        &self,
        chess: &Chess,
        test: Option<&Player>
    )-> Result<(), ChessError> {

        let mut player_to_move = chess.turn;

        if let Some(p) = test {
            player_to_move = *p;
        }

        if Player::find_player(&self.piece) != player_to_move {
            return Err(ChessError::PieceBelongsToOpponent)
        }

        if let false = self.is_valid_destination(chess, player_to_move) {
            return Err(ChessError::InvalidDestination)
        }

        if let false = self.piece.possible_moves(&self.from).contains(&self) {
            return Err(ChessError::NotAllowedMove)
        }

        if let ChessState::Check {
            checked_player: _,
            moves_left
        } = &chess.state {
            if let false = moves_left.contains(&self) {
                return Err(ChessError::NotAllowedMoveInCheck)
            }
        }

        if self.piece != Piece::Knight(chess.turn) {
            if let false = chess.path_is_clear(&self.from, &self.to) {
                return  Err(ChessError::PathIsBlocked);
            }
        }

        Ok(())
    }

    fn specific_tests(
        &self,
        chess: &Chess
    )-> Result<MoveType, ChessError> {

        let move_type = MoveType::determine_type(chess, &self);

        let result = match move_type {
            MoveType::Castle => self.castling_tests(chess),
            MoveType::EnPassant => self.en_passant_tests(chess),
            MoveType::PawnEat => self.pawn_eat_tests(chess),
            MoveType::PawnTwo => self.pawn_two_tests(),
            MoveType::PawnOne => self.pawn_one_tests(chess),
            _ => Ok(())
        };

        if let Err(e) = result {
            return Err(e)
        }

        Ok(move_type)
    }

    fn castling_tests(
        &self,
        chess: &Chess
    ) -> Result<(), ChessError> {

        let (king_from, rook_from) = match chess.turn {
            Player::White => if self.to == (0, 2) {
                ((0, 4), (0, 0))
            } else {
                ((0, 4), (0, 7))
            },
            Player::Black => if self.to == (7, 2) {
                ((7, 4), (7, 0))
            } else {
                ((7, 4), (7, 7))
            }
        };

        if self.from != king_from {
            return Err(ChessError::NotAllowedMove)
        }
        
        if let true = chess.moves
            .iter()
            .any(|m| m.from == king_from || m.from == rook_from ) {
            
            return Err(ChessError::CastlingMoveUnavailable)
        }

        let castling_path = chess.find_path(&king_from, &rook_from);

        if let Some(_) = chess.piece_can_move_to(&chess.turn.opponent(), castling_path, false) {
            return Err(ChessError::CastlingPathIsCompromised)
        }

        Ok(())
    }

    fn en_passant_tests(
        &self,
        chess: &Chess
    ) -> Result<(), ChessError> {

        let opp_pawn = Piece::Pawn(chess.turn.opponent());
        let adjacent_tiles = [(self.to.0 - 1, self.to.1), (self.to.0 + 1, self.to.1)];

        if let false = adjacent_tiles.into_iter().any(|(x, y)| {
            if let Some(last_move) = chess.moves.last() {
                chess.board[x][y] == Some(opp_pawn) && last_move.to == (x, y)
            } else {
                false
            }
        }) {
            return Err(ChessError::NotAllowedMove)
        }

        Ok(())
    }

    fn pawn_eat_tests(
        &self,
        chess: &Chess
    ) -> Result<(), ChessError> {

        if chess.board[self.to.0][self.to.1] == None {
            return Err(ChessError::InvalidDestination)
        }

        Ok(())
    }

    fn pawn_two_tests(
        &self
    ) -> Result<(), ChessError> {

        let player = Player::find_player(&self.piece);

        if player == Player::White && self.from.0 != 1 || player == Player::Black && self.from.0 != 6 {
            return Err(ChessError::NotAllowedMove)
        }

        Ok(())
    }

    fn pawn_one_tests(
        &self,
        chess: &Chess
    ) -> Result<(), ChessError> {

        if chess.board[self.to.0][self.to.1] != None {
            return Err(ChessError::NotAllowedMove)
        }

        Ok(())
    }

    pub fn finalise_move(
        &self,
        chess: &mut Chess,
        move_type: &MoveType
    ) -> ChessState {

        let mut new_piece = None;

        match move_type {
            MoveType::Promotion => {
                println!("Your pawn reached the last row.");
                loop {
                    println!("Choose the piece you want to change it to - 'QUEEN', 'ROOK', 'BISHOB', 'KNIGHT' - or 'NONE' if no change.");
                
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).expect("Failed to read line");

                    new_piece = match s.trim().as_ref() {
                        "QUEEN" => Some(Piece::Queen(chess.turn)),
                        "ROOK" => Some(Piece::Rook(chess.turn)),
                        "BISHOB" => Some(Piece::Bishob(chess.turn)),
                        "KNIGHT" => Some(Piece::King(chess.turn)),
                        "NONE" => Some(Piece::Pawn(chess.turn)),
                        _ => {
                            println!("Invalid input, try again.");
                            continue
                        }
                    };
                    break
                }
            },
            MoveType::Castle => {
                let (rook_from, rook_to) = match self.to {
                    (0, 2) => ((0, 0), (0, 3)),
                    (0, 6) => ((0, 7), (0, 5)),
                    (7, 2) => ((7, 0), (7, 3)),
                    _ => ((7, 7), (7, 5))
                };
                chess.board[rook_from.0][rook_from.1] = None;
                chess.board[rook_to.0][rook_to.1] = Some(Piece::Rook(chess.turn));
            },
            MoveType::EnPassant => {
                let opp_pawn_pos = if self.to.0 > self.from.0 {
                    (self.to.0 - 1,self.to.1)
                } else {
                    (self.to.0 + 1, self.to.1)
                };
                chess.board[opp_pawn_pos.0][opp_pawn_pos.1] = None;
            },
            _ => ()
        }

        if *move_type == MoveType::Promotion {
            chess.board[self.from.0][self.from.1] = None;
            chess.board[self.to.0][self.to.1] = new_piece;
        } else {
            chess.board[self.from.0][self.from.1] = None;
            chess.board[self.to.0][self.to.1] = Some(self.piece);
        }

        chess.moves.push(*self);

        let is_check = chess.is_check(&chess.turn, None);

        if let Some(moves_left) = chess.moves_left(is_check) {
            if moves_left.is_empty() {
                ChessState::Checkmate
            } else {
                ChessState::Check {
                    checked_player: chess.turn.opponent(),
                    moves_left
                }
            }
        } else {
            ChessState::Normal
        }
    }
}
