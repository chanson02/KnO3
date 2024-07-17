use super::GameState;
use std::cmp::{min, max};
use std::collections::HashSet;

impl GameState {

    /// Move squares in iterator until a piece is hit
    fn move_until_piece<I>(&self, range: I, white: bool) -> Vec<u8>
    where I: Iterator<Item = u8> {
        let mut result = Vec::new();
        let own = self.board.one_side_pieces(white);
        let opps = self.board.one_side_pieces(!white);

        for square in range {
            let btwise = 1 << square;
            if own & btwise != 0 { break; }

            result.push(square);
            if opps & btwise != 0 { break; }
        }

        result
    }

    pub fn possible_moves(&self, square: u8) -> Option<Vec<u8>> {
        let piece = match self.board.piece_at_position(square) {
            Some(p) => p,
            None => return None
        };

        let is_white = piece.is_ascii_uppercase();

        let possible_moves = match piece.to_ascii_lowercase() {
            'p' => self.possible_pawn_moves(square, is_white),
            'r' => self.possible_rook_moves(square, is_white),
            'n' => self.possible_knight_moves(square, is_white),
            'b' => self.possible_bishop_moves(square, is_white),
            'k' => self.possible_king_moves(square, is_white),
            'q' => self.possible_queen_moves(square, is_white),
            _ => return None
        };
        Some(possible_moves)
    }

    // TODO: en passant check
    fn possible_pawn_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();
        let rank = from % 8;
        let direction = if white { 1 } else { -1 };
        let initial_rank = if white { 1 } else { 6 };

        let opps = self.board.one_side_pieces(!white);
        let taken = self.board.both_side_pieces();
        
        let left_diag = from as i32 + 7 * direction;
        let forward = from as i32 + 8 * direction;
        let right_diag = from as i32 + 9 * direction;

        if taken & 1 << forward != 0 {
            result.push(forward as u8);
            if rank == initial_rank {
                let double = forward + 8 * direction;
                if taken & 1 << double != 0 { result.push(double as u8); }
            }
        }

        if left_diag >= 0 && left_diag <= 63 && opps & (1 << left_diag) != 0 {
            result.push(left_diag as u8);
        }
        if right_diag >= 0 && right_diag <= 63 && opps & (1 << right_diag) != 0 {
            result.push(right_diag as u8);
        }

        result.retain(|&square| (0..=63).contains(&square));
        result
    }

    fn possible_rook_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();
        let left_bound = from - from % 8;
        let right_bound = left_bound + 7;

        result.extend(self.move_until_piece((left_bound..from).rev(), white)); // leftward moves
        result.extend(self.move_until_piece(from+1..=right_bound, white)); // rightwards 
        result.extend(self.move_until_piece( // upward
                (from + 8..=63).step_by(8),
                white
        ));
        result.extend(self.move_until_piece( // downward
                (0..from).step_by(8).rev(),
                white
        ));

        result
    }

    fn possible_bishop_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();

        let rank = from % 8; // how many rows we can move right
        let nw_bound = min(56, from + rank * 7);
        let sw_bound = max(0, from - rank * 9);

        let inv_rank = 7 - rank; // inverse rank (how many rows we can move left)
        let ne_bound = min(63, from + inv_rank * 9);
        let se_bound = max(7, from - inv_rank * 7);

        let nw = (from + 7..=nw_bound).step_by(7);
        let sw = (sw_bound..from).rev().step_by(9);
        let ne = (from + 9..=ne_bound).step_by(9);
        let se = (se_bound..from).rev().step_by(7);

        result.extend(self.move_until_piece(nw, white));
        result.extend(self.move_until_piece(sw, white));
        result.extend(self.move_until_piece(ne, white));
        result.extend(self.move_until_piece(se, white));

        result
    }

    fn possible_queen_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = self.possible_rook_moves(from, white);
        result.extend(self.possible_bishop_moves(from, white));
        result
    }

    // TODO: Make sure they are not moving into check/mate
    fn possible_king_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let mut result = Vec::new();
        let directions: [i8; 8] = [-1, 1, -7, 7, -8, 8, -9, 9];
        let own = self.board.one_side_pieces(white);

        for &direction in &directions {
            let target = from as i8 + direction;
            if target >= 0 && target <= 63 {
                if own & (1 << target) == 0 {
                    result.push(target as u8);
                }
            }
        }

        result
    }

    fn possible_knight_moves(&self, from: u8, white: bool) -> Vec<u8> {
        let rank = from % 8;
        let own = self.board.one_side_pieces(white);

        let mut moves = HashSet::from([-6, 6, -10, 10, -15, 15, -17, 17]);
        let north = HashSet::from([ 6,  10,  15,  17]);
        let south = HashSet::from([-6, -10, -15, -17]);
        let east  = HashSet::from([-6,  10, -15,  17]);
        let west  = HashSet::from([ 6, -10,  15, -17]);
        let horz: HashSet<i32> = east.union(&west).cloned().collect();
        let vert: HashSet<i32> = north.union(&south).cloned().collect();

        if from < 8       { moves = moves.difference(&south).cloned().collect(); }
        else if from > 55 { moves = moves.difference(&north).cloned().collect(); }
        else if from < 16 {
            moves = moves.difference(
                &south.difference(&horz).cloned().collect()
            ).cloned().collect();
        }
        else if from > 47 {
            moves = moves.difference(
                &north.difference(&horz).cloned().collect()
            ).cloned().collect();
        }

        if rank == 0      { moves = moves.difference(&west).cloned().collect(); }
        else if rank == 7 { moves = moves.difference(&east).cloned().collect(); }
        else if rank == 1 {
            moves = moves.difference(
                &west.difference(&vert).cloned().collect()
            ).cloned().collect();
        }
        else if rank == 6 {
            moves = moves.difference(
                &east.difference(&vert).cloned().collect()
            ).cloned().collect();
        }

        let mut result = Vec::new();
        for &mve in &moves {
            let target = from as i32 + mve;
            if target >= 0 && target <= 63 {
                if own & (1 << target) == 0 {
                    result.push(target as u8);
                }
            }
        }
        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_moves() {
        let mut gs = GameState::new();
        assert_eq!(gs.possible_pawn_moves(17, true), vec![25]); // normal move
        assert_eq!(gs.possible_pawn_moves(9, true), vec![17, 25]); // beginning move
        assert_eq!(gs.possible_pawn_moves(49, false), vec![41, 33]); // beginning move
        assert_eq!(gs.possible_pawn_moves(42, true), vec![49, 51]); // capture

        // out of bounds
        assert_eq!(gs.possible_pawn_moves(57, true), vec![]);
        assert_eq!(gs.possible_pawn_moves(1, false), vec![]);

        assert_eq!(gs.possible_pawn_moves(1, true), vec![]); // behind another white piece
        gs.board.black_pawns |= 1 << 8; // place a black pawn on 8
        assert_eq!(gs.possible_pawn_moves(1, true), vec![8]); // should be able to take 8 now
    }

    #[test]
    fn test_rook_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_rook_moves(0, true), vec![]); // blocked
        assert_eq!(gs.possible_rook_moves(33, true), vec![32, 34, 35, 36, 37, 38, 39, 25, 17, 41, 49]); // normal move
    }

    #[test]
    fn test_bishop_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_bishop_moves(2, true), vec![]); // blocked
        assert_eq!(gs.possible_bishop_moves(34, true), vec![41, 48, 43, 52, 27, 20, 25, 16]);
    }

    #[test]
    fn test_knight_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_knight_moves(1, true), vec![16, 18]); // white left starting
        assert_eq!(gs.possible_knight_moves(6, true), vec![21, 23]); // white right starting
        assert_eq!(gs.possible_knight_moves(34, true), vec![49, 51, 44, 28, 19, 17, 24, 40]); // normal move
        assert_eq!(gs.possible_knight_moves(62, false), vec![45, 47]); // black right starting
    }

    #[test]
    fn test_king_moves() {
        let gs = GameState::new();
        assert_eq!(gs.possible_king_moves(4, true), vec![]); // blocked
        // TODO: this will fail when checking is added
        assert_eq!(gs.possible_king_moves(4, false), vec![3, 11, 12, 13, 14, 6]);
        assert_eq!(gs.possible_king_moves(34, true), vec![41, 42, 43, 33, 35, 25, 26, 27]); // normal move
    }
}
