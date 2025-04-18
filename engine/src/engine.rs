use crate::position;

// Encourages central pawn advances and penalizes isolated/doubled pawns
const PAWN_TABLE: [f64; 64] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
    0.5, // Reduced center penalties
    0.5, -0.5, -1.0, 0.0, 0.0, -1.0, -0.5, 0.5, 0.0, 0.0, 0.0, 2.5, 2.5, 0.0, 0.0,
    0.0, // Increased center bonus
    0.5, 0.5, 1.0, 3.0, 3.0, 1.0, 0.5, 0.5, // Enhanced advanced pawns
    1.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0,
];

// Rewards centralization and flexible outposts
const KNIGHT_TABLE: [f64; 64] = [
    -5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0, -4.0, -2.0, 0.0, 0.5, 0.5, 0.0, -2.0,
    -4.0, // Better 7th rank
    -3.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -3.0, // Stronger center
    -3.0, 0.5, 2.0, 2.5, 2.5, 2.0, 0.5, -3.0, // Prime central bonus
    -3.0, 0.0, 2.0, 2.5, 2.5, 2.0, 0.0, -3.0, -3.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -3.0, -4.0, -2.0,
    0.0, 0.5, 0.5, 0.0, -2.0, -4.0, -5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0,
];

// Favors long diagonals and central control
const BISHOP_TABLE: [f64; 64] = [
    -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0, -1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0, -1.0,
    0.5, 1.0, 1.5, 1.5, 1.0, 0.5, -1.0, // Enhanced diagonals
    -1.0, 0.5, 1.5, 2.0, 2.0, 1.5, 0.5, -1.0, -1.0, 0.0, 1.5, 2.0, 2.0, 1.5, 0.0, -1.0, -1.0, 1.0,
    1.5, 1.5, 1.5, 1.5, 1.0, -1.0, -1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, -1.0, -2.0, -1.0, -1.0,
    -1.0, -1.0, -1.0, -1.0, -2.0,
];

// Encourages 7th rank control and open files
const ROOK_TABLE: [f64; 64] = [
    0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.5, 1.5, 1.5, 1.5, 1.5, 1.5,
    1.5, 0.5, // Stronger 7th rank
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

// Balances central control with king safety
const QUEEN_TABLE: [f64; 64] = [
    -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0, -1.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, -1.0, -1.0,
    0.0, 0.75, 0.75, 0.75, 0.75, 0.0, -1.0, // Better center
    -0.5, 0.0, 0.75, 1.0, 1.0, 0.75, 0.0, -0.5, 0.0, 0.0, 0.75, 1.0, 1.0, 0.75, 0.0, -0.5, -1.0,
    0.5, 0.75, 0.75, 0.75, 0.75, 0.0, -1.0, -1.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0,
    -1.0, -0.5, -0.5, -1.0, -1.0, -2.0,
];

// Emphasizes castling safety and endgame centralization
const KING_TABLE: [f64; 64] = [
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0, -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0, -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
    -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0, -1.0, -2.0, -2.0, -2.5, -2.5, -2.0, -2.0,
    -1.0, // Safer castling
    2.5, 2.5, 0.0, -0.5, -0.5, 0.0, 2.5, 2.5, // Castle wing bonus
    2.0, 3.5, 1.5, 0.0, 0.0, 1.5, 3.5, 2.0, // Endgame centralization
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
            // Add cases for bishops, rooks, queens, and kings.
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