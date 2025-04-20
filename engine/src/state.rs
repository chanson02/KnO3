use super::Chessboard;
use crate::engine::evaluate_position;

impl Chessboard {
    pub fn piece_bitboards(&self) -> [(char, u64); 12] {
        [
            ('P', self.white_pawns),
            ('N', self.white_knights),
            ('B', self.white_bishops),
            ('K', self.white_king),
            ('Q', self.white_queen),
            ('R', self.white_rooks),
            ('p', self.black_pawns),
            ('n', self.black_knights),
            ('b', self.black_bishops),
            ('k', self.black_king),
            ('q', self.black_queen),
            ('r', self.black_rooks),
        ]
    }

    pub fn piece_bitboard(&mut self, piece: char) -> Result<&mut u64, String> {
        match piece {
            'p' => Ok(&mut self.black_pawns),
            'r' => Ok(&mut self.black_rooks),
            'n' => Ok(&mut self.black_knights),
            'b' => Ok(&mut self.black_bishops),
            'k' => Ok(&mut self.black_king),
            'q' => Ok(&mut self.black_queen),
            'P' => Ok(&mut self.white_pawns),
            'R' => Ok(&mut self.white_rooks),
            'N' => Ok(&mut self.white_knights),
            'B' => Ok(&mut self.white_bishops),
            'K' => Ok(&mut self.white_king),
            'Q' => Ok(&mut self.white_queen),
            _ => Err(format!("Invalid piece type: {piece}")),
        }
    }

    pub fn piece_at_position(&self, square: u8) -> Option<char> {
        let btwise = 1 << square;
        for (p_type, positions) in self.piece_bitboards() {
            if btwise & positions != 0 {
                return Some(p_type);
            }
        }
        None
    }

    pub fn one_side_pieces(&self, white: bool) -> u64 {
        if white {
            self.white_bishops
                | self.white_king
                | self.white_knights
                | self.white_pawns
                | self.white_rooks
                | self.white_queen
        } else {
            self.black_bishops
                | self.black_king
                | self.black_knights
                | self.black_pawns
                | self.black_rooks
                | self.black_queen
        }
    }

    pub fn both_side_pieces(&self) -> u64 {
        self.one_side_pieces(true) | self.one_side_pieces(false)
    }

    /// Determine who is winning
    /// A positive number indicates white is winning
    pub fn evaluate(&self) -> i64 {
        let mut result = 0;

        for (piece, board) in self.piece_bitboards() {
            let score = match piece.to_ascii_uppercase() {
                'P' => 1,
                'R' => 5,
                'N' => 3,
                'B' => 3,
                'Q' => 9,
                _ => 0,
            } * board.count_ones() as i64;

            if piece.is_ascii_uppercase() {
                result += score;
                result += evaluate_position(self, true) as i64;
            } else {
                result -= score;
                result += evaluate_position(self, false) as i64
            }
        }

        // Add positional evaluation.
        let positional_score = evaluate_position(self, true) - evaluate_position(self, false);
        result += positional_score as i64;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_at_position() {
        let cb = Chessboard {
            white_pawns: 0x0100,
            black_knights: 0x00400000,
            ..Chessboard::empty()
        };

        assert_eq!(cb.piece_at_position(8), Some('P'));
        assert_eq!(cb.piece_at_position(22), Some('n'));
        assert_eq!(cb.piece_at_position(32), None);
    }

    #[test]
    fn test_evaluate_initial_position() {
        let cb = Chessboard::new();
        let evaluation = cb.evaluate();
        assert_eq!(evaluation, 0, "Initial position should be balanced.");
    }

    #[test]
    fn test_evaluate_white_advantage() {
        let cb = Chessboard {
            white_queen: 0x0000000000000008,
            black_queen: 0x0000000000000000,
            ..Chessboard::empty()
        };
        let evaluation = cb.evaluate();
        assert!(
            evaluation > 0,
            "White should have an advantage with an extra queen."
        );
    }

    #[test]
    fn test_evaluate_black_advantage() {
        let cb = Chessboard {
            white_queen: 0x0000000000000000,
            black_queen: 0x0800000000000000,
            ..Chessboard::empty()
        };
        let evaluation = cb.evaluate();
        assert!(
            evaluation < 0,
            "Black should have an advantage with an extra queen."
        );
    }

    #[test]
    fn test_evaluate_positional_advantage() {
        let cb = Chessboard {
            white_pawns: 0x000000000000FF00, // Pawns advanced to rank 3
            black_pawns: 0x00FF000000000000, // Pawns on rank 7
            ..Chessboard::empty()
        };
        let evaluation = cb.evaluate();
        assert!(
            evaluation > 0,
            "White should have a positional advantage with advanced pawns."
        );
    }
}
