use super::position;
use super::Chessboard;

impl Chessboard {
    pub fn get_score(&self) -> i32 {
        let mut score = 0;

        // Material balance
        score += self.white_pawns.count_ones() as i32 - self.black_pawns.count_ones() as i32;
        score +=
            3 * (self.white_knights.count_ones() as i32 - self.black_knights.count_ones() as i32);
        score +=
            3 * (self.white_bishops.count_ones() as i32 - self.black_bishops.count_ones() as i32);
        score += 5 * (self.white_rooks.count_ones() as i32 - self.black_rooks.count_ones() as i32);
        score += 9 * (self.white_queen.count_ones() as i32 - self.black_queen.count_ones() as i32);

        // TODO: Take into account amount of legal moves per piece
        score
    }

    // I would like to clean up the nesting in here --Cooper
    /// This function does not validate that there is a pawn at this position
    pub fn get_pawn_moves(&self, from: i64, white: bool) -> Vec<i64> {
        let rank = position::square_to_rank(from);
        let direction = if white { 1 } else { -1 };
        let initial_rank = if white { 2 } else { 7 };

        let mut result = Vec::new();
        let left_diag = from + 7 * direction;
        let forward = from + 8 * direction;
        let right_diag = from + 9 * direction;

        if self.piece_at_position(forward).is_none() {
            result.push(forward);
            if rank == initial_rank {
                let double = from + 16 * direction;
                if self.piece_at_position(double).is_none() {
                    result.push(double);
                }
            }
        }

        let opponent_pieces = self.one_side_pieces(!white);
        if opponent_pieces & (1 << left_diag) != 0 { result.push(left_diag); }
        if opponent_pieces & (1 << right_diag) != 0 { result.push(right_diag); }

        result.retain(|&square| (0..=63).contains(&square)); // stay within bounds
        result
    }
}
