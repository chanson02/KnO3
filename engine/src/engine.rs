use crate::{position, Chessboard, GameState};

#[rustfmt::skip]
const PAWN_TABLE: [f64; 64] = [
    -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67,
    -0.50, -0.33, -0.33, -1.00, -1.00, -0.33, -0.33, -0.50,
    -0.50, -0.83, -1.00, -0.67, -0.67, -1.00, -0.83, -0.50,
    -0.67, -0.67, -0.67, 0.17, 0.17, -0.67, -0.67, -0.67,
    -0.50, -0.50, -0.33, 0.33, 0.33, -0.33, -0.50, -0.50,
    -0.33, -0.33, 0.00, 0.33, 0.33, 0.00, -0.33, -0.33,
    1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00,
    -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67,
];

const KNIGHT_TABLE: [f64; 64] = [
    -1.00, -0.73, -0.47, -0.47, -0.47, -0.47, -0.73, -1.00, -0.73, -0.20, 0.33, 0.47, 0.47, 0.33,
    -0.20, -0.73, -0.47, 0.33, 0.73, 0.87, 0.87, 0.73, 0.33, -0.47, -0.47, 0.47, 0.87, 1.00, 1.00,
    0.87, 0.47, -0.47, -0.47, 0.33, 0.87, 1.00, 1.00, 0.87, 0.33, -0.47, -0.47, 0.47, 0.73, 0.87,
    0.87, 0.73, 0.47, -0.47, -0.73, -0.20, 0.33, 0.47, 0.47, 0.33, -0.20, -0.73, -1.00, -0.73,
    -0.47, -0.47, -0.47, -0.47, -0.73, -1.00,
];

const BISHOP_TABLE: [f64; 64] = [
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00, -0.50, 0.00, 0.25, 0.25, 0.25, 0.25,
    0.00, -0.50, -0.50, 0.25, 0.50, 0.75, 0.75, 0.50, 0.25, -0.50, -0.50, 0.25, 0.75, 1.00, 1.00,
    0.75, 0.25, -0.50, -0.50, 0.00, 0.75, 1.00, 1.00, 0.75, 0.00, -0.50, -0.50, 0.50, 0.75, 0.75,
    0.75, 0.75, 0.50, -0.50, -0.50, 0.25, 0.00, 0.00, 0.00, 0.00, 0.25, -0.50, -1.00, -0.50, -0.50,
    -0.50, -0.50, -0.50, -0.50, -1.00,
];

const ROOK_TABLE: [f64; 64] = [
    -0.50, -0.50, -0.50, 0.00, 0.00, -0.50, -0.50, -0.50, -1.00, -0.50, -0.50, -0.50, -0.50, -0.50,
    -0.50, -1.00, -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00, -1.00, -0.50, -0.50,
    -0.50, -0.50, -0.50, -0.50, -1.00, -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00, 0.00, 1.00, 1.00, 1.00, 1.00, 1.00,
    1.00, 0.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50,
];

const QUEEN_TABLE: [f64; 64] = [
    -1.00, -0.33, -0.33, 0.00, 0.00, -0.33, -0.33, -1.00, -0.33, 0.33, 0.67, 0.67, 0.67, 0.67,
    0.33, -0.33, -0.33, 0.33, 0.83, 0.83, 0.83, 0.83, 0.33, -0.33, 0.00, 0.33, 0.83, 1.00, 1.00,
    0.83, 0.33, 0.00, 0.33, 0.33, 0.83, 1.00, 1.00, 0.83, 0.33, 0.00, -0.33, 0.67, 0.83, 0.83,
    0.83, 0.83, 0.33, -0.33, -0.33, 0.33, 0.67, 0.33, 0.33, 0.33, 0.33, -0.33, -1.00, -0.33, -0.33,
    0.00, 0.00, -0.33, -0.33, -1.00,
];

const KING_TABLE: [f64; 64] = [
    -0.53, -0.76, -0.76, -1.00, -1.00, -0.76, -0.76, -0.53, -0.53, -0.76, -0.76, -1.00, -1.00,
    -0.76, -0.76, -0.53, -0.53, -0.76, -0.76, -1.00, -1.00, -0.76, -0.76, -0.53, -0.53, -0.76,
    -0.76, -1.00, -1.00, -0.76, -0.76, -0.53, -0.29, -0.53, -0.53, -0.76, -0.76, -0.53, -0.53,
    -0.29, -0.06, -0.29, -0.29, -0.41, -0.41, -0.29, -0.29, -0.06, 0.76, 0.76, 0.18, 0.06, 0.06,
    0.18, 0.76, 0.76, 0.65, 1.00, 0.53, 0.18, 0.18, 0.53, 1.00, 0.65,
];

/// Evaluate the positional value of the board for white or black.
/// Positive scores favor white, negative scores favor black.
pub fn evaluate_position(board: &Chessboard, is_white: bool) -> f64 {
    let mut score = 0.0;

    for (piece, bitboard) in board.piece_bitboards() {
        // Skip pieces that don't belong to the current player.
        if !is_white && piece.is_ascii_uppercase() || is_white && piece.is_ascii_lowercase() {
            continue;
        }

        // Select the appropriate piece-square table.
        let table = match piece.to_ascii_uppercase() {
            'P' => &PAWN_TABLE,
            'N' => &KNIGHT_TABLE,
            'B' => &BISHOP_TABLE,
            'R' => &ROOK_TABLE,
            'Q' => &QUEEN_TABLE,
            'K' => &KING_TABLE,
            _ => continue,
        };

        // Determine the score multiplier based on the player's color.
        let multiplier = if !is_white { 1.0 } else { -1.0 };

        // Calculate the score contribution for each active square.
        for square in position::active_squares(bitboard) {
            let table_index = if is_white {
                square as usize
            } else {
                63 - square as usize // Mirror the table for black pieces.
            };
            score += multiplier * table[table_index];
        }
    }

    score
}

/// Implements the minimax algorithm to determine the best move.
/// `depth` specifies the depth of the search tree.
/// Returns the best move and its evaluation score.
pub fn minimax(
    game_state: &mut GameState,
    depth: u8,
    is_maximizing: bool,
    alpha: f64,
    beta: f64,
) -> (Option<(u8, u8)>, f64) {
    //TODO: is_game_over() is taken care of in a different issue
    // if depth == 0 || game_state.is_game_over() {
    //     return (None, evaluate_position(&game_state.board, is_maximizing));
    // }

    let mut best_move = None;
    let mut best_score = if is_maximizing {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };
    let mut alpha = alpha;
    let mut beta = beta;

    for from in 0..64 {
        let possible_moves = game_state.possible_moves(from);
        for to in position::active_squares(possible_moves) {
            if !game_state.is_move_legal(from, to) {
                continue;
            }
            let mut new_state = game_state.clone();
            new_state.move_piece(from, to);

            let (_, score) = minimax(&mut new_state, depth - 1, !is_maximizing, alpha, beta);

            if is_maximizing {
                if score > best_score {
                    best_score = score;
                    best_move = Some((from, to));
                }
                alpha = alpha.max(score);
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = Some((from, to));
                }
                beta = beta.min(score);
            }

            if beta <= alpha {
                break;
            }
        }
    }

    (best_move, best_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chessboard::Chessboard, GameState};

    #[test]
    fn test_evaluate_position_white() {
        let board = Chessboard::new();
        let score = evaluate_position(&board, true);
        assert!(
            score > 0.0,
            "Expected positive score for white's initial position"
        );
    }

    #[test]
    fn test_evaluate_position_black() {
        let board = Chessboard::new();
        let score = evaluate_position(&board, false);
        assert!(
            score < 0.0,
            "Expected negative score for black's initial position"
        );
    }

    #[test]
    fn test_evaluate_position_empty_board() {
        let board = Chessboard::empty();
        let white_score = evaluate_position(&board, true);
        let black_score = evaluate_position(&board, false);
        assert_eq!(
            white_score, 0.0,
            "Expected score of 0 for white on an empty board"
        );
        assert_eq!(
            black_score, 0.0,
            "Expected score of 0 for black on an empty board"
        );
    }

    #[test]
    fn test_evaluate_position_mirrored_board() {
        let mut board = Chessboard::empty();
        board.white_pawns = 0xFF00; // White pawns on rank 2
        board.black_pawns = 0xFF000000000000; // Black pawns on rank 7
        let white_score = evaluate_position(&board, true);
        let black_score = evaluate_position(&board, false);
        assert_eq!(
            white_score, -black_score,
            "Expected mirrored scores for mirrored board"
        );
    }

    #[test]
    fn test_minimax_initial_position() {
        let game_state = GameState::new();
        let (best_move, score) = minimax(&game_state, 3, true, f64::NEG_INFINITY, f64::INFINITY);
        assert!(best_move.is_some(), "Expected a valid move");
        assert!(score > f64::NEG_INFINITY, "Expected a valid score");
    }

    #[test]
    fn test_minimax_empty_board() {
        let mut game_state = GameState::new();
        game_state.board = Chessboard::empty();
        let (best_move, score) = minimax(&game_state, 3, true, f64::NEG_INFINITY, f64::INFINITY);
        assert!(best_move.is_none(), "Expected no valid moves");
        assert_eq!(score, 0.0, "Expected score of 0 for empty board");
    }

    #[test]
    fn test_minimax_one_move() {
        let mut game_state = GameState::new();
        game_state.board.white_pawns = 0x0000000000000100; // White pawn on E2
        game_state.board.black_pawns = 0x0000000000000000; // No black pawns
        let (best_move, score) = minimax(&game_state, 1, true, f64::NEG_INFINITY, f64::INFINITY);
        assert_eq!(best_move, Some((12, 28)), "Expected move E2 to E4");
        assert!(score > 0.0, "Expected positive score for white");
    }
}
