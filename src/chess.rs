use std::io;
use crate::{
    r#move::{
        Move,
        MoveType
    },
    player::Player,
    piece::Piece,
    constant::{
        INIT_BOARD,
        DIAGONALS,
        STRAIGHTS
    }
};

#[derive(Debug, PartialEq, Clone)]
pub enum ChessState {
    Checkmate,
    Check {
        checked_player: Player,
        moves_left: Vec<Move>
    },
    Normal
}

#[derive(Debug, Clone)]
pub struct Chess {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Player,
    pub state: ChessState,
    pub moves: Vec<Move>
}

impl Chess {
    pub fn new() -> Chess {

        Chess {
            board: INIT_BOARD,
            turn: Player::White,
            state: ChessState::Normal,
            moves: Vec::new()
        }
    }

    pub fn play(&mut self) {

        println!("To move a piece, type it's current and new spot (e.g 'A1 A2')");

        let mut turn: u8 = 1;

        loop {

            println!("{}", self);

            if turn % 2 == 1 {
                self.turn = Player::White;
            } else {
                self.turn = Player::Black;
            }
            println!("{}'s turn.", self.turn);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let m = match Move::from_input(&self, input) {
                Ok(m) => m,
                Err(e) => {
                    println!("{}", e);
                    continue
                }
            };

            match m.process_move(self, None)  {
                Ok(state) => match state {
                    ChessState::Checkmate => {
                        println!("Checkmate! {} won the game.", self.turn);
                        println!("{}", self);
                        break
                    },
                    ChessState::Check {
                        checked_player,
                        moves_left: _
                    } => {
                        println!("{} is in a check.", checked_player);
                        self.state = state;
                    },
                    ChessState::Normal => { self.state = state; }
                },
                Err(e) => {
                    println!("{}", e);
                    continue
                }
            }

            turn += 1;
        }
    }

    pub fn is_check(
        &self,
        checking_player: &Player,
        test: Option<&Move>
    ) -> Option<Vec<Move>> {

        let mut king_tile = self.find_king(&checking_player.opponent());

        if let Some(m) = test {

            let mut test_chess = self.clone();

            test_chess.board[m.from.0][m.from.1] = None;
            test_chess.board[m.to.0][m.to.1] = Some(m.piece);

            king_tile = test_chess.find_king(&checking_player.opponent());

            return test_chess.piece_can_move_to(&checking_player, vec![king_tile], false)
        }

        self.piece_can_move_to(&checking_player, vec![king_tile], false)
    }

    pub fn piece_can_move_to(
        &self,
        player: &Player,
        tiles: Vec<(usize, usize)>,
        is_check_block: bool
    ) -> Option<Vec<Move>> {

        let checking_moves: Vec<Move> = (0..8)
            .flat_map(|x| (0..8).filter_map(move |y| match self.board[x][y] {
                Some(piece) => if Piece::get_pieces(player)[0..=4].contains(&piece) {
                    Some((piece, (x, y)))
                } else {
                    None
                },
                None => None
            }))
            .map(|(p, from)| p.possible_moves(&from) )
            .flat_map(|moves| moves )
            .filter_map(|m| if let Ok(move_type) = m.is_valid_move(&mut self.clone(), Some(player)) {
                if is_check_block { // any move to block the check
                    Some(m)
                } else { // moves that compromise king or castling path
                    match move_type {
                        MoveType::PawnEat | MoveType::Other => Some(m),
                        _ => None
                    }
                }
            } else {
                None
            })
            .filter(|m| tiles
                .iter()
                .any(|t| *t == m.to )
            )
            .collect()
        ;
        
        if let false = checking_moves.is_empty() {
            Some(checking_moves)
        } else {
            None
        }
    }

    pub fn find_path(
        &self,
        from: &(usize, usize),
        to: &(usize, usize)
    ) -> Vec<(usize, usize)> {

        let (from, to) = ((from.0 as i8, from.1 as i8), (to.0 as i8, to.1 as i8));
        let m = ((to.0 - from.0), (to.1 - from.1));

        [DIAGONALS, STRAIGHTS]
            .iter()
            .flat_map(|a| a )
            .filter(|a| a.contains(&m) )
            .flat_map(|a| a )
            .take_while(|i| *i != &m )
            .map(|m| {
                let m = (from.0 + m.0, from.1 + m.1);
                (m.0 as usize, m.1 as usize)
            })
            .collect()
    }

    pub fn path_is_clear(
        &self,
        from: &(usize, usize),
        to: &(usize, usize)
    ) -> bool {

        self.find_path(from, to)
         .iter()
         .all(|(x, y)| self.board[*x][*y] == None )
    }

    pub fn find_king(
        &self,
        player: &Player
    ) -> (usize, usize) {

        (0..8)
            .fold((0, 0), |mut acc, x| {
                (0..8).for_each(|y| if self.board[x][y] == Some(Piece::King(*player)) {
                    acc = (x, y);
                });
                acc
            })
    }

    pub fn moves_left(
        &self,
        is_check: Option<Vec<Move>>
    ) -> Option<Vec<Move>> {

        if let Some(checking_moves) = is_check {

            let mut moves_left: Vec<Move> = Vec::new();

            let player = self.turn.opponent();

            Piece::King(player)
                .possible_moves(&checking_moves[0].to)
                .iter()
                .for_each(|m| if m.is_valid_move(&mut self.clone(), Some(&player)).is_ok() {
                    if self.is_check(&self.turn, Some(m)).is_none() {
                        moves_left.push(*m)
                    }
                })
            ;

            checking_moves
                .iter()
                .for_each(|m| {

                    let path = self.find_path(&m.from, &m.to);
                    let checking_piece_tile = vec![m.from];

                    let results = [
                        self.piece_can_move_to(&player, path, true),
                        self.piece_can_move_to(&player, checking_piece_tile, false)
                    ];

                    results
                        .iter()
                        .for_each(|r| {
                            if let Some(moves) = r {
                                moves
                                    .iter()
                                    .filter(|m| m.is_valid_move(&mut self.clone(), Some(&player)).is_ok() )
                                    .for_each(|m| moves_left.push(*m) )
                                ;
                            }
                        })
                    ;
                })
            ;

            Some(moves_left)
        } else {
            None
        }
    }

    pub fn find_piece(
        &self,
        tile: &(usize, usize)
    ) -> Option<Piece> {

        if let Some(piece) = self.board[tile.0][tile.1] {

            let players = [Player::White, Player::Black];

            players
                .iter()
                .fold(None, |mut acc, player|{

                    match Piece::get_pieces(player).iter().find(|p| *p == &piece ) {
                        Some(p) => { acc = Some(*p); },
                        None => ()
                    }

                    acc
                })
        } else {
            None
        }
    }
}