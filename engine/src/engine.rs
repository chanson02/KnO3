use crate::position;

const PAWN_TABLE: [f64; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 0.5, 0.5, -0.5,
    -1.0, 0.0, 0.0, -1.0, -0.5, 0.5, 0.0, 0.0, 0.0, 2.5, 2.5, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 3.0,
    3.0, 1.0, 0.5, 0.5, 1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0,
    5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

const KNIGHT_TABLE: [f64; 64] = [
    -5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0, -4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0, -4.0,
    -3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0, -3.0, 0.5, 2.0, 2.5, 2.5, 2.0, 0.5, -3.0, -3.0, 0.0,
    2.0, 2.5, 2.5, 2.0, 0.0, -3.0, -3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0, -4.0, -2.0, 0.0, 0.5,
    0.5, 0.0, -2.0, -4.0, -5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0,
];

const BISHOP_TABLE: [f64; 64] = [
    -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0, -1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0, -1.0,
    0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -1.0, -1.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -1.0, -1.0, 0.0, 1.5,
    2.0, 2.0, 1.5, 0.0, -1.0, -1.0, 1.0, 1.5, 1.5, 1.5, 1.5, 1.0, -1.0, -1.0, 0.5, 0.0, 0.0, 0.0,
    0.0, 0.5, -1.0, -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0,
];

const ROOK_TABLE: [f64; 64] = [
    0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.5, 1.5, 1.5, 1.5, 1.5, 1.5,
    1.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

const QUEEN_TABLE: [f64; 64] = [
    -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0, -1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0, -1.0,
    0.0, 0.75, 0.75, 0.75, 0.75, 0.0, -1.0, -0.5, 0.0, 0.75, 1.0, 1.0, 0.75, 0.0, -0.5, 0.0, 0.0,
    0.75, 1.0, 1.0, 0.75, 0.0, -0.5, -1.0, 0.5, 0.75, 0.75, 0.75, 0.75, 0.0, -1.0, -1.0, 0.0, 0.5,
    0.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0,
];

const KING_TABLE: [f64; 64] = [
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0, -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0, -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0, -1.0, -2.0, -2.0, -2.5, -2.5, -2.0, -2.0, -1.0,
    2.5, 2.5, 0.0, -0.5, -0.5, 0.0, 2.5, 2.5, 2.0, 3.5, 1.5, 0.0, 0.0, 1.5, 3.5, 2.0,
];

/// Evaluate the positional value of the board for white or black.
// Ensure the Chessboard type is imported or defined
use crate::chessboard::Chessboard; // Adjust the path based on your project structure

pub fn evaluate_position(board: &Chessboard, is_white: bool) -> f64 {
    let mut score = 0.0;

    for (piece, bitboard) in board.piece_bitboards() {
        let table = match piece.to_ascii_uppercase() {
            'P' => &PAWN_TABLE,
            'N' => &KNIGHT_TABLE,
            'B' => &BISHOP_TABLE,
            'R' => &ROOK_TABLE,
            'Q' => &QUEEN_TABLE,
            'K' => &KING_TABLE,
            _ => continue,
        };

        let is_piece_white = piece.is_ascii_uppercase();
        let multiplier = if is_piece_white == is_white {
            1.0
        } else {
            -1.0
        };

        for square in position::active_squares(bitboard) {
            let table_index = if is_piece_white {
                square as usize
            } else {
                63 - square as usize // Mirror the table for black pieces.
            };
            score += multiplier * table[table_index];
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chessboard::Chessboard;

    #[test]
    fn test_evaluate_position_white() {
        let board = Chessboard::new();
        let score = evaluate_position(&board, true);
        assert!(score > 0.0, "Expected positive score for white's initial position");
    }

    #[test]
    fn test_evaluate_position_black() {
        let board = Chessboard::new();
        let score = evaluate_position(&board, false);
        assert!(score < 0.0, "Expected negative score for black's initial position");
    }

    #[test]
    fn test_evaluate_position_empty_board() {
        let board = Chessboard::empty();
        let white_score = evaluate_position(&board, true);
        let black_score = evaluate_position(&board, false);
        assert_eq!(white_score, 0.0, "Expected score of 0 for white on an empty board");
        assert_eq!(black_score, 0.0, "Expected score of 0 for black on an empty board");
    }

    #[test]
    fn test_evaluate_position_mirrored_board() {
        let mut board = Chessboard::empty();
        board.white_pawns = 0xFF00; // White pawns on rank 2
        board.black_pawns = 0xFF000000000000; // Black pawns on rank 7
        let white_score = evaluate_position(&board, true);
        let black_score = evaluate_position(&board, false);
        assert_eq!(white_score, -black_score, "Expected mirrored scores for mirrored board");
    }
}
