use crate::{position, Chessboard, GameState};

/// Evaluate the positional score from the perspective of one player.
/// Positive scores indicate white has a more favorable position.
pub fn evaluate_position(board: &Chessboard, is_white: bool) -> f64 {
    let mut score = 0.0;

    for (piece, bitboard) in board.piece_bitboards() {
        // Skip pieces that don't belong to the current player.
        if !is_white && piece.is_ascii_uppercase() || is_white && piece.is_ascii_lowercase() {
            continue;
        }

        // Get the appropriate piece evaluation table.
        let table = match position::get_piece_evaluation_table(piece) {
            Some(t) => t,
            None => continue,
        };

        // Calculate the score contribution for each active square.
        for square in position::active_squares(bitboard) {
            let table_index = if is_white {
                square
            } else {
                63 - square // Mirror the table for black pieces.
            } as usize;
            let piece_score = table[table_index];
            score += piece_score;
        }
    }
    if !is_white {
        score
    } else {
        -score
    }
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
        print!("White score: {}", score);
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
    fn test_evaluate_position_new_board() {
        let board = Chessboard::new();
        let white_score = evaluate_position(&board, true);
        let black_score = evaluate_position(&board, false);
        assert_eq!(
            white_score, -black_score,
            "Expected mirrored scores for new board"
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

    // #[test]
    // fn test_evaluate_position_pawn_advance() {
    //     let mut board = Chessboard::empty();
    //     board.white_pawns = 0xFF00; // White pawns on rank 2
    //     print!("Initial board: {:?}", board);
    //     let initial_score = evaluate_position(&board, true);

    //     // Advance a white pawn from e2 to e4
    //     board.white_pawns = 0x0800F700; // Remove pawn from e2 and place it on e4
    //     print!("Updated board: {:?}", board);
    //     let updated_score = evaluate_position(&board, true);
    //     print!("Initial score: {}, Updated score: {}", initial_score, updated_score);
    //     assert!(
    //         updated_score > initial_score,
    //         "Expected score to improve after advancing a pawn"
    //     );
    // }
}
