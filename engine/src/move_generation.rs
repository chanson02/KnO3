use crate::position;

use super::GameState;
use std::cmp::{max, min};

impl GameState {
    pub fn move_piece(&mut self, from: u8, to: u8) {
        let piece = match self.board.piece_at_position(from) {
            None => return,
            Some(p) => p,
        };
        let piece_type = piece.to_ascii_lowercase();

        // Clear position we're landing on
        if let Some(piece) = self.board.piece_at_position(to) {
            let to_piece_bitboard = self
                .board
                .piece_bitboard(piece)
                .expect("we already validate this");
            *to_piece_bitboard &= !(1 << to);
        }

        // Clear enemy pawn if en passanted
        if piece_type == 'p' && to == self.en_passant {
            let enemy_pawns = if self.white_turn {
                self.board.piece_bitboard('p').expect("Black pawns")
            } else {
                self.board.piece_bitboard('P').expect("White pawns")
            };
            *enemy_pawns &= !(1
                << if self.white_turn {
                    to.saturating_sub(8)
                } else {
                    to + 8
                });
        }

        self.en_passant = 255;
        let from_piece_bitboard = self
            .board
            .piece_bitboard(piece)
            .expect("we already validate this");
        *from_piece_bitboard &= !(1 << from); // Clear position coming from
        *from_piece_bitboard |= 1 << to; // Set position landing on

        // check en passant
        if piece.eq_ignore_ascii_case(&'p') {
            let en_passant_square = (from + to) / 2;
            if from + 8 == en_passant_square || from.saturating_sub(8) == en_passant_square {
                self.en_passant = en_passant_square;
            }
        }
    }

    pub fn move_piece_legally(&mut self, from: u8, to: u8) -> Result<(), String> {
        // Check if it's this pieces turn
        if let Some(piece) = self.board.piece_at_position(from) {
            if self.white_turn == piece.is_ascii_lowercase() {
                return Err("Cannot move piece on opponents turn".to_string());
            }
        }

        // Check if this piece
        let possible_moves = self.possible_moves(from);
        if 1 << to & possible_moves == 0 {
            let from_string = position::square_to_string(from);
            let to_string = position::square_to_string(to);
            return Err(format!("{from_string} -> {to_string} illegal move"));
        }

        self.move_piece(from, to);
        self.white_turn = !self.white_turn;
        Ok(())
    }

    /// Move squares in iterator until a piece is hit
    fn move_until_piece<I>(&self, range: I, white: bool) -> u64
    where
        I: Iterator<Item = u8>,
    {
        let mut result = 0;
        let own = self.board.one_side_pieces(white);
        let opps = self.board.one_side_pieces(!white);

        for square in range {
            let btwise = 1 << square;
            if own & btwise != 0 {
                break;
            }

            result |= btwise;
            if opps & btwise != 0 {
                break;
            }
        }

        result
    }

    pub fn possible_moves(&self, square: u8) -> u64 {
        let piece = match self.board.piece_at_position(square) {
            Some(p) => p,
            None => return 0,
        };

        let is_white = piece.is_ascii_uppercase();
        match piece.to_ascii_lowercase() {
            'p' => self.possible_pawn_moves(square, is_white),
            'r' => self.possible_rook_moves(square, is_white),
            'n' => self.possible_knight_moves(square, is_white),
            'b' => self.possible_bishop_moves(square, is_white),
            'k' => self.possible_king_moves(square, is_white),
            'q' => self.possible_queen_moves(square, is_white),
            _ => 0,
        }
    }

    fn possible_pawn_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;
        let rank = (from / 8) as i8;
        let file = (from % 8) as i8;
        let direction: i8 = if white { 1 } else { -1 };
        let initial_rank = if white { rank == 1 } else { rank == 6 };
        let next_rank = rank + direction;
        if !(0..8).contains(&next_rank) {
            return result;
        }

        let opps = self.board.one_side_pieces(!white);
        let taken = self.board.both_side_pieces();
        let forward = (next_rank * 8 + file) as u8;

        if taken & (1 << forward) == 0 {
            result |= 1 << forward;
            if initial_rank {
                let double = ((rank + 2 * direction) * 8 + file) as u8;
                if taken & (1 << double) == 0 {
                    result |= 1 << double;
                }
            }
        }

        for file_offset in [-1, 1] {
            let target_file = file + file_offset;
            if !(0..8).contains(&target_file) {
                continue;
            }
            let target = (next_rank * 8 + target_file) as u8;
            if opps & (1 << target) != 0 || (!initial_rank && target == self.en_passant) {
                result |= 1 << target;
            }
        }

        result
    }

    fn possible_rook_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;
        let file = from % 8;
        let left_bound = from - file;
        let right_bound = left_bound + 7;

        result |= self.move_until_piece((left_bound..from).rev(), white); // west moves
        result |= self.move_until_piece(from + 1..=right_bound, white); // east moves
        result |= self.move_until_piece((from + 8..=63).step_by(8), white); // north moves
        result |= self.move_until_piece((file..from).step_by(8).rev(), white); // south moves

        result
    }

    fn rook_attack_map(&self, pos: u8, white: bool) -> u64 {
        let rank = pos / 8;
        let mut result = 0;
        let possible_attacks =
            self.possible_rook_moves(pos, white) & self.board.one_side_pieces(!white);

        // Find the furthest move in each direction (north, east, south, west)
        let north = !((1 << (pos + 1)) - 1); // everything above `pos` is 1
        let south = (1 << pos) - 1; // everything below `pos` is 1
        let horz = 0xFF << (rank * 8); // everything on the same rank as `pos`
        let east = horz & north;
        let west = horz & south;

        let north_most = (possible_attacks & north).leading_zeros();
        if north_most < 64 {
            result |= 1 << (63 - north_most);
        }

        let south_most = (possible_attacks & south).trailing_zeros();
        if south_most < 64 {
            result |= 1 << south_most;
        }

        let east_most = (possible_attacks & east).leading_zeros();
        if east_most < 64 {
            result |= 1 << (63 - east_most);
        }

        let west_most = (possible_attacks & west).trailing_zeros();
        if west_most < 64 {
            result |= 1 << west_most;
        }

        result
    }

    fn possible_bishop_moves(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;

        let file = from % 8; // how many rows we can move right
        let nw_bound = min(56, from + file * 7);
        let sw_bound = from.saturating_sub(file * 9);

        let inv_file = 7 - file; // inverse rank (how many rows we can move left)
        let ne_bound = min(63, from + inv_file * 9);
        let se_bound = max(0, from.saturating_sub(inv_file * 7));

        let nw = (from + 7..=nw_bound).step_by(7);
        let sw = (sw_bound..=from.saturating_sub(9)).rev().step_by(9);
        let ne = (from + 9..=ne_bound).step_by(9);
        let se = (se_bound..=from.saturating_sub(7)).rev().step_by(7);

        result |= self.move_until_piece(nw, white);
        result |= self.move_until_piece(sw, white);
        result |= self.move_until_piece(ne, white);
        result |= self.move_until_piece(se, white);

        result
    }

    fn bishop_attack_map(&self, pos: u8, white: bool) -> u64 {
        let mut result = 0;
        let file = pos % 8;
        let possible_attacks =
            self.possible_bishop_moves(pos, white) & self.board.one_side_pieces(!white);

        let west_of_file = (1 << file) - 1_u64;
        let east_of_file = 0xFF & !((1 << (file + 1)) - 1_u64);

        let west = possible_attacks
            & (west_of_file
                | west_of_file << 8
                | west_of_file << (8 * 2)
                | west_of_file << (8 * 3)
                | west_of_file << (8 * 4)
                | west_of_file << (8 * 5)
                | west_of_file << (8 * 6)
                | west_of_file << (8 * 7));

        let east = possible_attacks
            & (east_of_file
                | east_of_file << 8
                | east_of_file << (8 * 2)
                | east_of_file << (8 * 3)
                | east_of_file << (8 * 4)
                | east_of_file << (8 * 5)
                | east_of_file << (8 * 6)
                | east_of_file << (8 * 7));

        let ne_most = east.leading_zeros();
        if ne_most < 64 {
            result |= 1 << (63 - ne_most);
        }

        let nw_most = west.leading_zeros();
        if nw_most < 64 {
            result |= 1 << (63 - nw_most);
        }

        let se_most = east.trailing_zeros();
        if se_most < 64 {
            result |= 1 << se_most;
        }

        let sw_most = west.trailing_zeros();
        if sw_most < 64 {
            result |= 1 << sw_most;
        }

        result
    }

    fn possible_queen_moves(&self, from: u8, white: bool) -> u64 {
        self.possible_rook_moves(from, white) | self.possible_bishop_moves(from, white)
    }

    fn possible_king_moves_ignore_check(&self, from: u8, white: bool) -> u64 {
        let mut result = 0;
        let rank = (from / 8) as i8;
        let file = (from % 8) as i8;
        let directions: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let own = self.board.one_side_pieces(white);

        for &(d_rank, d_file) in &directions {
            let target_rank = rank + d_rank;
            let target_file = file + d_file;
            if !(0..8).contains(&target_rank) || !(0..8).contains(&target_file) {
                continue;
            }

            let target = (target_rank * 8 + target_file) as u8;
            if own & (1 << target) == 0 {
                result |= 1 << target;
            }
        }

        result
    }

    fn possible_king_moves(&self, from: u8, white: bool) -> u64 {
        let mut all_moves = self.possible_king_moves_ignore_check(from, white);
        let mut filtered = 0;

        // Filter out squares under attack
        while all_moves != 0 {
            let to = all_moves.trailing_zeros() as u8;
            let mask = 1 << to;
            if !self.position_under_attack(to, white) {
                filtered |= mask;
            }
            all_moves &= all_moves - 1;
        }

        // check for castling possibility
        let capital_k = self.castling & 0b1000 != 0;
        let capital_q = self.castling & 0b0100 != 0;
        let k = self.castling & 0b0010 != 0;
        let q = self.castling & 0b0001 != 0;

        // check which side we can castle too
        let king_side = capital_k & white || k & !white;
        let queen_side = capital_q & white || q & !white;
        if (!king_side & !queen_side) || self.position_under_attack(from, white) {
            return filtered;
        }

        let shift = 8 * (from / 8);
        let greater_than = !((1u64 << (from + 1)) - 1); // everything above `pos` is 1
        let less_than = (1u64 << from) - 1; // everything below `pos` is 1
        let horz = (0xFF << shift) & self.board.both_side_pieces(); // everything on the same rank as `pos`
        let east = horz & greater_than;
        let west = horz & less_than;

        let blocked_k_side = !king_side || east & (0b01100000 << shift) != 0;
        let blocked_q_side = !queen_side || west & (0b00001110 << shift) != 0;
        if !blocked_k_side
            && !self.position_under_attack(from + 1, white)
            && !self.position_under_attack(from + 2, white)
        {
            filtered |= 0b01000000 << shift;
        }
        if !blocked_q_side
            && !self.position_under_attack(from - 1, white)
            && !self.position_under_attack(from - 2, white)
        {
            filtered |= 0b00000100 << shift;
        }

        filtered //all moves the king can make
    }

    fn king_attack_map_ignore_check(&self, pos: u8, white: bool) -> u64 {
        self.possible_king_moves_ignore_check(pos, white) & self.board.one_side_pieces(!white)
    }

    fn possible_knight_moves(&self, from: u8, white: bool) -> u64 {
        let file = from % 8;
        let rank = from / 8;
        let own = self.board.one_side_pieces(white);
        let mut result = 0;

        let moves: [(i8, i8); 8] = [
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (-2, -1),
            (-2, 1),
            (2, -1),
            (2, 1),
        ];

        for (d_rank, d_file) in moves.iter() {
            let n_rank = d_rank + rank as i8;
            let n_file = d_file + file as i8;
            if !(0..8).contains(&n_rank) || !(0..8).contains(&n_file) {
                continue;
            }

            let target = (n_rank * 8 + n_file) as u8;
            if own & (1 << target) == 0 {
                result |= 1 << target;
            }
        }

        result
    }

    fn knight_attack_map(&self, pos: u8, white: bool) -> u64 {
        self.possible_knight_moves(pos, white) & self.board.one_side_pieces(!white)
    }

    /// Can this square be taken by the opponent next turn?
    fn position_under_attack(&self, square: u8, white: bool) -> bool {
        let file = square % 8;
        let opp_rooks = if white {
            self.board.black_rooks | self.board.black_queen
        } else {
            self.board.white_rooks | self.board.white_queen
        };
        let opp_bish = if white {
            self.board.black_bishops | self.board.black_queen
        } else {
            self.board.white_bishops | self.board.white_queen
        };
        let opp_knights = if white {
            self.board.black_knights
        } else {
            self.board.white_knights
        };
        let opp_king = if white {
            self.board.black_king
        } else {
            self.board.white_king
        };
        let opp_pawn = if white {
            self.board.black_pawns
        } else {
            self.board.white_pawns
        };

        let pawn_attack_map = if white {
            if square >= 48 {
                0
            } else {
                let mut map = 0;
                if file != 0 {
                    map |= 1 << (square + 7);
                }
                if file != 7 {
                    map |= 1 << (square + 9);
                }
                map
            }
        } else if square <= 15 {
            0
        } else {
            let mut map = 0;
            if file != 0 {
                map |= 1 << (square - 9);
            }
            if file != 7 {
                map |= 1 << (square - 7);
            }
            map
        };

        self.rook_attack_map(square, white) & opp_rooks != 0
            || self.bishop_attack_map(square, white) & opp_bish != 0
            || self.knight_attack_map(square, white) & opp_knights != 0
            || self.king_attack_map_ignore_check(square, white) & opp_king != 0
            || pawn_attack_map & opp_pawn != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Chessboard;

    use super::*;

    #[test]
    fn test_pawn_moves() {
        let mut gs = GameState::new();
        assert_eq!(
            gs.possible_pawn_moves(17, true),
            1 << 25,
            "Failed normal move"
        );
        assert_eq!(
            gs.possible_pawn_moves(9, true),
            1 << 17 | 1 << 25,
            "Failed beginning move"
        );
        assert_eq!(
            gs.possible_pawn_moves(49, false),
            1 << 41 | 1 << 33,
            "Failed black start move"
        );
        assert_eq!(
            gs.possible_pawn_moves(42, true),
            1 << 49 | 1 << 51,
            "Failed white->black capture"
        );

        // out of bounds
        assert_eq!(gs.possible_pawn_moves(57, true), 0, "Failed white oob");
        assert_eq!(gs.possible_pawn_moves(1, false), 0, "Failed black oob");

        gs.en_passant = 16;
        assert_eq!(
            gs.possible_pawn_moves(9, true),
            1 << 17 | 1 << 25,
            "En-passanted own piece"
        );
        gs.en_passant = 24;
        assert_eq!(
            gs.possible_pawn_moves(17, true),
            1 << 24 | 1 << 25,
            "Did not en passant"
        );

        assert_eq!(gs.possible_pawn_moves(1, true), 0, "Moved behind own piece");
        gs.board.black_pawns |= 1 << 8; // place a black pawn on 8
        assert_eq!(gs.possible_pawn_moves(1, true), 1 << 8, "Failed capture");

        gs.board = Chessboard::empty();
        gs.board.white_pawns = 1 << 8;
        gs.board.black_pawns = 1 << 15 | 1 << 17;
        assert_eq!(
            gs.possible_pawn_moves(8, true),
            1 << 16 | 1 << 24 | 1 << 17,
            "White pawn wrapped across board edge"
        );

        gs.board = Chessboard::empty();
        gs.board.black_pawns = 1 << 55;
        gs.board.white_pawns = 1 << 40 | 1 << 46;
        assert_eq!(
            gs.possible_pawn_moves(55, false),
            1 << 47 | 1 << 39 | 1 << 46,
            "Black pawn wrapped across board edge"
        );
    }

    #[test]
    fn test_rook_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_rook_moves(0, true), 0, "Captured own");
        assert_eq!(
            gs.possible_rook_moves(33, true),
            1 << 32
                | 1 << 34
                | 1 << 35
                | 1 << 36
                | 1 << 37
                | 1 << 38
                | 1 << 39
                | 1 << 41
                | 1 << 49
                | 1 << 25
                | 1 << 17,
            "Failed normal move"
        );
    }

    #[test]
    fn test_rook_attacks() {
        let mut gs = GameState::new();
        assert_eq!(gs.rook_attack_map(8, true), 1 << 48, "Did not attack north");
        assert_eq!(
            gs.rook_attack_map(49, false),
            1 << 9,
            "Did not attack south"
        );

        gs.board = Chessboard::empty();
        assert_eq!(gs.rook_attack_map(0, true), 0, "Attacking nothing");

        gs.board.black_pawns |= 1 << 1;
        assert_eq!(gs.rook_attack_map(0, true), 1 << 1, "Did not attack east");
        assert_eq!(gs.rook_attack_map(7, true), 1 << 1, "Did not attack west");

        // surround a rook with pieces
        let pawns = 1 << 1 | 1 << 8 | 1 << 17 | 1 << 10;
        gs.board.black_pawns = pawns;
        assert_eq!(
            gs.rook_attack_map(9, true),
            pawns,
            "Did not attack all directions"
        );
    }

    #[test]
    fn test_bishop_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_bishop_moves(2, true), 0, "Captured own");
        assert_eq!(
            gs.possible_bishop_moves(34, true),
            1 << 41 | 1 << 48 | 1 << 25 | 1 << 16 | 1 << 43 | 1 << 52 | 1 << 27 | 1 << 20,
            "Failed normal move"
        );
    }

    #[test]
    fn test_bishop_attacks() {
        let mut gs = GameState::new();
        gs.board = Chessboard::empty();

        assert_eq!(gs.bishop_attack_map(0, true), 0, "Attacking nothing");

        gs.board.black_pawns |= 1 << 18;
        assert_eq!(gs.bishop_attack_map(0, true), 1 << 18);
        gs.board.black_pawns |= 1 << 9;
        assert_eq!(
            gs.bishop_attack_map(0, true),
            1 << 9,
            "Attacked through piece"
        );

        let pawns = 1 << 16 | 1 << 18 | 1 | 1 << 2;
        gs.board.black_pawns = pawns;
        assert_eq!(
            gs.bishop_attack_map(9, true),
            pawns,
            "Did not attack all directions"
        );

        gs.board.black_pawns = 0;
        gs.board.white_pawns = pawns;
        assert_eq!(
            gs.bishop_attack_map(9, false),
            pawns,
            "Black did not attack all directions"
        );
    }

    #[test]
    fn test_knight_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_knight_moves(1, true), 1 << 16 | 1 << 18); // white left starting
        assert_eq!(
            gs.possible_knight_moves(1, false),
            1 << 11 | 1 << 16 | 1 << 18
        ); // black taking
        assert_eq!(gs.possible_knight_moves(6, true), 1 << 21 | 1 << 23); // white right starting
        assert_eq!(
            gs.possible_knight_moves(34, true),
            1 << 24 | 1 << 28 | 1 << 40 | 1 << 44 | 1 << 17 | 1 << 19 | 1 << 49 | 1 << 51,
            "Failed normal move"
        );
        assert_eq!(gs.possible_knight_moves(62, false), 1 << 45 | 1 << 47); // black right starting
    }

    #[test]
    fn test_king_moves() {
        let mut gs = GameState::new();
        gs.castling = 0;
        assert_eq!(gs.possible_king_moves(4, true), 0, "Captured own");

        assert_eq!(
            gs.possible_king_moves_ignore_check(4, false),
            1 << 3 | 1 << 5 | 1 << 11 | 1 << 12 | 1 << 13
        );
        assert_eq!(gs.possible_king_moves(4, false), 0, "Moved into check");

        assert_eq!(
            gs.possible_king_moves(34, true),
            1 << 33 | 1 << 35 | 1 << 27 | 1 << 26 | 1 << 25,
            "Failed normal move"
        );

        gs.board = Chessboard::empty();
        gs.castling = 0;
        assert_eq!(
            gs.possible_king_moves_ignore_check(7, true),
            1 << 6 | 1 << 14 | 1 << 15,
            "King wrapped across board edge"
        );
    }

    #[test]
    fn test_castling() {
        let mut gs = GameState::new();
        gs.board = Chessboard::empty();
        gs.castling = 0b1111;

        assert!(
            gs.possible_king_moves(4, true) & (1 << 6) != 0,
            "White king did not castle king side"
        );
        assert!(
            gs.possible_king_moves(4, true) & (1 << 2) != 0,
            "White king did not castle queen side"
        );
        assert!(
            gs.possible_king_moves(60, false) & (1 << 62 | 1 << 58) != 0,
            "Black king did not castle"
        );

        gs.board.black_rooks = 1 << 13;
        gs.castling = 0b1001;
        assert!(
            gs.possible_king_moves(4, true) & (1 << 6) == 0,
            "Castled through attack"
        );
        assert!(
            gs.possible_king_moves(4, true) & (1 << 2) == 0,
            "White castled after piece moved"
        );
        assert!(
            gs.possible_king_moves(60, false) & (1 << 62) == 0,
            "Black castled after piece moved"
        );
    }

    #[test]
    fn test_move_until_piece() {
        let gs = GameState::new();
        let itr = (18..=58).step_by(8);
        assert_eq!(
            gs.move_until_piece(itr.clone(), true),
            1 << 18 | 1 << 26 | 1 << 34 | 1 << 42 | 1 << 50,
            "White should move in straight line to black"
        );
        assert_eq!(
            gs.move_until_piece(itr, false),
            1 << 18 | 1 << 26 | 1 << 34 | 1 << 42,
            "Black should move to next black"
        );

        assert_eq!(gs.move_until_piece(0..7, true), 0);
        assert_eq!(gs.move_until_piece(0..7, false), 1); // can eat this piece
    }

    #[test]
    fn test_possible_moves() {
        let mut gs = GameState::new();
        assert_eq!(gs.possible_moves(9), gs.possible_pawn_moves(9, true)); // pawn begin
        assert_eq!(gs.possible_moves(42), 0); // nothing here
        gs.board.white_pawns |= 1 << 42;
        assert_eq!(gs.possible_moves(42), gs.possible_pawn_moves(42, true)); // pawn eat
        assert_eq!(gs.possible_moves(50), gs.possible_pawn_moves(50, false)); // black pawn blocked

        gs.board.white_pawns = 0;
        gs.board.black_pawns = 0;
        assert_eq!(gs.possible_moves(0), gs.possible_rook_moves(0, true));
        assert_eq!(gs.possible_moves(56), gs.possible_rook_moves(56, false));
        assert_eq!(gs.possible_moves(1), gs.possible_knight_moves(1, true));
        assert_eq!(gs.possible_moves(57), gs.possible_knight_moves(57, false));
        assert_eq!(gs.possible_moves(2), gs.possible_bishop_moves(2, true));
        assert_eq!(gs.possible_moves(58), gs.possible_bishop_moves(58, false));
        assert_eq!(gs.possible_moves(3), gs.possible_queen_moves(3, true));
        assert_eq!(gs.possible_moves(59), gs.possible_queen_moves(59, false));
        assert_eq!(gs.possible_moves(4), gs.possible_king_moves(4, true));
        assert_eq!(gs.possible_moves(60), gs.possible_king_moves(60, false));
    }

    #[test]
    fn test_position_under_attack() {
        let mut gs = GameState::new();
        gs.board = Chessboard::empty();

        assert!(!gs.position_under_attack(9, true), "Attacked by nothing");

        gs.board.black_pawns = 1 << 16;
        assert!(gs.position_under_attack(9, true), "Pawn not attacking");
        gs.board.black_pawns = 1 << 2;
        assert!(
            !gs.position_under_attack(9, true),
            "Black pawn attacking backwards"
        );
        gs.board.black_pawns = 0;

        gs.board.black_queen = 1 << 57;
        assert!(gs.position_under_attack(9, true), "Queen not attacking");
        gs.board.black_queen = 0;

        gs.board.black_rooks = 1 << 49;
        assert!(gs.position_under_attack(9, true), "Rook not attacking");
        gs.board.black_rooks = 0;

        gs.board.black_bishops = 1 << 18;
        assert!(gs.position_under_attack(9, true), "Bishop not attacking");
        gs.board.black_bishops = 0;

        gs.board.black_knights = 1 << 26;
        assert!(gs.position_under_attack(9, true), "Knight not attacking");
    }

    #[test]
    fn test_move_piece() {
        let mut gs = GameState::new();

        gs.move_piece(12, 28);
        assert_eq!(
            gs.board.piece_at_position(12),
            None,
            "Pawn still exists in old position"
        );
        assert_eq!(
            gs.board.piece_at_position(28),
            Some('P'),
            "Pawn does not exist in new position"
        );

        gs.move_piece(28, 52);
        assert!(
            gs.board.black_pawns & (1 << 52) == 0,
            "Captured piece not overwritten"
        );

        assert_eq!(
            gs.board.piece_at_position(24),
            None,
            "24 should be empty for the next test to pass"
        );
        gs.move_piece(24, 32);
        assert_eq!(gs.board.piece_at_position(32), None, "Empty square moved");
    }

    #[test]
    fn test_en_passant() {
        let mut gs = GameState::new();
        assert!(gs.en_passant > 63, "New game stated with en passant");
        gs.move_piece(8, 24);
        assert_eq!(gs.en_passant, 16, "White pawn did not set en passant");
        gs.move_piece(48, 32);
        assert_eq!(gs.en_passant, 40, "Black pawn did not set en passant");
        gs.move_piece(32, 24);
        assert!(gs.en_passant > 63, "En passant did not reset");

        gs.board = Chessboard::empty();
        gs.board.black_pawns |= 1 << 32;
        gs.board.white_pawns |= 1 << 33;
        gs.en_passant = 40;
        gs.move_piece(33, 40);
        assert!(
            gs.board.black_pawns & (1 << 32) == 0,
            "En Passanted pawn not cleared after capture"
        );
    }

    #[test]
    fn test_move_piece_legally() {
        let mut gs = GameState::new();
        let mve = gs.move_piece_legally(12, 28);

        assert!(mve.is_ok(), "Expected white pawn to move 12 -> 28");
        assert_eq!(
            gs.board.piece_at_position(12),
            None,
            "Pawn still exists in old position"
        );
        assert_eq!(
            gs.board.piece_at_position(28),
            Some('P'),
            "Pawn does not exist in new position"
        );
        assert!(
            !gs.white_turn,
            "Turn did not change to black after white moved"
        );

        gs.white_turn = true;
        let illegal_move = gs.move_piece_legally(28, 12);
        assert!(illegal_move.is_err(), "Pawn illegally moved backwards");

        assert_eq!(
            gs.board.piece_at_position(24),
            None,
            "24 should be emtpy for next test to pass"
        );
        gs.white_turn = true;
        let illegal_move = gs.move_piece_legally(24, 16);
        assert!(illegal_move.is_err(), "Empty square moved");

        gs.white_turn = true;
        let illegal_move = gs.move_piece_legally(48, 40);
        assert!(illegal_move.is_err(), "Moved black on white turn");
        gs.white_turn = false;
        let legal_move = gs.move_piece_legally(48, 40);
        assert!(legal_move.is_ok(), "Cannot move black on blacks turn");
    }
}
