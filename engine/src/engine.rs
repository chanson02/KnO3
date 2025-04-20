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

/// Minimax algorithm to determine the best move.
pub fn minimax(
    game_state: &mut GameState,
    depth: u8,
    is_maximizing: bool,
) -> (f64, Option<(u8, u8)>) {
    if depth == 0 {
        let score = evaluate_position(&game_state.board, game_state.white_turn);
        return (score, None);
    }

    let mut best_score = if is_maximizing {
        f64::NEG_INFINITY
    } else {
        f64::INFINITY
    };
    let mut best_move = None;

    for from in 0..64 {
        if let Some(piece) = game_state.board.piece_at_position(from) {
            if (piece.is_ascii_uppercase() && !game_state.white_turn)
                || (piece.is_ascii_lowercase() && game_state.white_turn)
            {
                continue;
            }

            let possible_moves = position::active_squares(game_state.possible_moves(from));
            for to in possible_moves {
                let mut cloned_state = game_state.clone();
                if cloned_state.move_piece_legally(from, to).is_ok() {
                    let (score, _) = minimax(&mut cloned_state, depth - 1, !is_maximizing);
                    let is_better = if is_maximizing {
                        score > best_score
                    } else {
                        score < best_score
                    };
                    if is_better {
                        best_score = score;
                        best_move = Some((from, to));
                    }
                }
            }
        }
    }

    (best_score, best_move)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chessboard::Chessboard;

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
    fn test_minimax_best_move() {
        let mut game_state = GameState::new();
        let (_, best_move) = minimax(&mut game_state, 2, true);
        assert!(best_move.is_some(), "Expected a valid best move");
        let (from, to) = best_move.unwrap();
        println!(
            "Best move: {} -> {}",
            position::square_to_string(from),
            position::square_to_string(to)
        );
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
