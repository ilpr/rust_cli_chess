/*

Implementing the main Chess struct.

*/

use std::{
    io,
    collections::HashMap
};
use crate::{
    piece::{
        Piece,
        DIAGONALS,
        STRAIGHTS
    },
    player::Player,
    error::ChessError
};

#[derive(Clone)]
pub struct Chess {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Player,
    pub moves: Vec<((usize, usize), (usize, usize))>,
    pub king_positions: HashMap<Piece, (usize, usize)>,
    pub is_check: bool,
    pub moves_left: HashMap<(usize, usize), Vec<(usize, usize)>>, // moves left when in a check
    pub castle: HashMap<Player, Vec<(i8, i8)>> // castle moves left
}

impl Chess {
    pub fn new() -> Chess {

        Chess {
            board: [
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
            ],
            turn: Player::White,
            moves: Vec::new(),
            king_positions: HashMap::from([
                (Piece::King(Player::White), (0, 4)),
                (Piece::King(Player::Black), (7, 4))
            ]),
            is_check: false,
            moves_left: HashMap::new(),
            castle: HashMap::from([
                (Player::White, vec![(0, 2), (0, -2)]),
                (Player::Black, vec![(0, 2), (0, -2)])
            ])
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

            let (old, new) = match self.parse_input(input) {
                Ok((old, new)) => (old, new),
                Err(e) => {
                    println!("{}", e);
                    continue
                }
            };

            match Piece::process_move(self, old, new) {
                Ok(()) => if self.is_check == true {
                    if self.moves_left.is_empty() {
                        println!("Checkmate! {} won.", self.turn);
                        break
                    } else {
                        println!("{} is in a check.", self.is_check(true).unwrap().get_opponent())
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    continue
                }
            }

            turn += 1;
        }
    }

    pub fn get_path(
        &self,
        old: &(usize, usize),
        p_move: &(i8, i8)
    ) -> Result<Vec<(usize, usize)>, ChessError> {

        // If the path to new index is clear, returns the moves between old and new index.

        let path: Vec<(usize, usize)> = [DIAGONALS, STRAIGHTS]
            .iter()
            .flat_map(|a| a.iter() )
            .filter(|a| a.contains(&p_move) )
            .flat_map(|a| a.to_owned().into_iter() )
            .take_while(|m| m != p_move )
            .map(|m| {

                let (x, y) = (old.0 as i8 + m.0, old.1 as i8 + m.1);

                (x as usize, y as usize)
            })
            .collect()
        ;

        for (x, y) in &path {
            if let Some(_p) = self.board[*x][*y] {
                return Err(ChessError::InvalidMove)
            }
        }

        Ok(path)
    }

    pub fn parse_input(&self, input: String) -> Result<((usize, usize), (usize, usize)), ChessError> {

        let get_index = |s: &str| {

            let parsed = match s.chars().map(|c| c.to_string() ).collect::<Vec<String>>().as_slice() {
                [x, y] => (x.to_owned(), y.to_owned()),
                _ => return Err(ChessError::InvalidInput)
            };

            let x: usize = match (1usize..=8).position(|n| n == parsed.1.parse().unwrap_or(99) ) {
                Some(x) => x,
                None => return Err(ChessError::InvalidInput)
            };
            let y: usize = match ["A", "B", "C", "D", "E", "F", "G", "H"].iter().position(|l| l == &parsed.0 ) {
                Some(y) => y,
                None => return Err(ChessError::InvalidInput)
            };

            if (0..8).contains(&y) {
                Ok((x, y))
            } else {
                Err(ChessError::InvalidInput)
            }
        };

        match input.trim().split_whitespace().collect::<Vec<&str>>().as_slice() {
            [old, new] => Ok((
                match get_index(old) {
                    Ok(x) => x,
                    Err(e) => return Err(e)
                },
                match get_index(new) {
                    Ok(y) => y,
                    Err(e) => return Err(e)
                }
            )),
            _ => Err(ChessError::InvalidInput)
        }
    }

    pub fn change_king_position(
        &mut self,
        piece: &Piece,
        new: &(usize, usize)
    ) {

        match piece {
            Piece::King(_p) => {
                self.king_positions.insert(*piece, *new);
            },
            _ => ()
        }
    }

    pub fn change_castle_state(
        &mut self,
        piece: &Piece,
        old: &(usize, usize)
    ) {

        match piece {
            Piece::King(p) => {
                self.castle.insert(*p, vec![]);
            },
            Piece::Rook(p) => {
                self.castle.insert(
                    *p,
                    self.castle
                        .get(&p)
                        .unwrap()
                        .iter()
                        .filter_map(|m| {
                            let move_to_remove = match old {
                                (0, 0) | (7, 0) => (0, -2),
                                (0, 7) | (7, 7 )=> (0, 2),
                                _ => (0, 0)
                            };
                            if m != &move_to_remove {
                                Some(*m)
                            } else {
                                None
                            }
                        })
                        .collect()
                );
            },
            _ => ()
        }
    }

    pub fn is_check(&mut self, test: bool) -> Option<Player> {

        let checks = (0..8).fold(Vec::new(), |mut acc: Vec<(Player, (usize, usize), (i8, i8))>, x| {
            let check: Vec<(Player, (usize, usize), (i8, i8))> = (0..8)
                .filter_map(|y| match self.board[x][y] {
                    Some(piece) => match piece {
                        Piece::Queen(p) |
                        Piece::Rook(p) |
                        Piece::Bishob(p) |
                        Piece::Knight(p) |
                        Piece::Pawn(p) => Some((piece, p, y)),
                        _ => None
                    },
                    None => None
                })
                .filter_map(|(piece, p, y)| {

                    let king = self.king_positions
                        .get(&Piece::King(p.get_opponent()))
                        .unwrap()
                    ;

                    let checking_move: Vec<(i8, i8)> = piece.moves()
                        .into_iter()
                        .filter(|m| (x as i8 + m.0, y as i8 + m.1) == (king.0 as i8, king.1 as i8) )
                        .collect()
                    ;

                    if !checking_move.is_empty() {
                        Some((p, (x, y), checking_move[0]))
                    } else {
                        None
                    }
                })
                .filter(|(_p, (x, y), m)| self.get_path(&(*x, *y), m).is_ok() )
                .collect()
            ;

            match check.iter().next() {
                Some(m) => acc.push(*m),
                None => ()
            };

            acc
        });

        if checks.is_empty() {
            if test == true {
                None
            } else {
                self.is_check = false;

                None
            }
        } else {

            let checking_player = checks[0].0;

            if test == true {
                Some(checking_player)
            } else {
                self.is_check = true;
                self.moves_left = self.find_moves_left(checks);

                Some(checking_player)
            }
        }
    }

    pub fn test_check(
        &self,
        piece: &Piece,
        old: &(usize, usize),
        new: &(usize, usize)
    ) -> Option<Player> {

        let mut test_chess = self.clone();

        test_chess.board[new.0][new.1] = Some(*piece);
        test_chess.board[old.0][old.1] = None;

        if let Piece::King(_p) = piece {
            test_chess.change_king_position(piece, new);
        }

        test_chess.is_check(true)
    }

    fn find_moves_left(
        &self,
        checks: Vec<(Player, (usize, usize), (i8, i8))>
    ) -> HashMap<(usize, usize), Vec<(usize, usize)>> {

        checks
            .iter()
            .fold(HashMap::new(), |mut acc, (player, check, check_move)| {

                let moves_left: HashMap<(usize, usize), Vec<(usize, usize)>> = (0..8)
                    .flat_map(|x| (0..8)
                        .filter_map(move |y| {

                            let piece = match self.board[x][y] {
                                Some(piece) => piece,
                                None => return None
                            };
                            
                            if Piece::get_pieces(&player.get_opponent()).contains(&piece) {
                                Some((y, piece))
                            } else {
                                None
                            }
                        })
                        .filter_map(move |(y, piece)| {

                            let path = match self.get_path(&check, check_move) {
                                Ok(p) => p,
                                Err(_) => return None
                            };

                            let poss_moves: Vec<(usize, usize)> = match piece {
                                Piece::King(p) => {

                                    piece.moves()[0..=7]
                                        .iter()
                                        .filter_map(|m| {
                                            
                                            let new = (x as i8 + m.0, y as i8 + m.1);

                                            if (0..8).contains(&new.0) && (0..8).contains(&new.1) {
                                                Some((new.0 as usize, new.1 as usize))
                                            } else {
                                                None
                                            }
                                        })
                                        .filter(|new| self.test_check(&piece, &(x, y), &new).is_none() )
                                        .filter(|new| !Piece::get_pieces(&p).contains(&self.board[new.0][new.1].unwrap_or(Piece::Pawn(*player))) )
                                        .collect()
                                },
                                _ => {

                                    piece.moves()
                                        .iter()
                                        .fold(Vec::new(), |mut acc, m| {

                                            let new = (x as i8 + m.0, y as i8 + m.1);
                                            let new = (new.0 as usize, new.1 as usize);

                                            if path.iter().any(|p| new == *p ) || &new == check {
                                                acc.push(new);
                                            }

                                            acc
                                        })
                                }
                            };

                            if !poss_moves.is_empty() {
                                Some(((x, y), poss_moves))
                            } else {
                                None
                            }
                        })
                    )
                    .collect()
                ;

                for (k, v) in moves_left {
                    acc.insert(k, v);
                }
                
                acc
            })
    }
}
