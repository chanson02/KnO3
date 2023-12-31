use crate::board::{position, Chessboard};
use regex::Regex;

/// Places pieces on the chessboard based on the Forsyth-Edwards Notation (FEN) rows provided.
/// 
/// # Arguments
/// 
/// - `chessboard`: A mutable reference to the `Chessboard` struct to update with the piece placement.
/// - `fen_rows`: A string representing the piece placement part of the FEN string.
#[rustfmt::skip]
pub fn place_pieces(chessboard: &mut Chessboard, fen_rows: &str) {
    chessboard.clear();
    for (row_index, row_string) in fen_rows.split('/').rev().enumerate() {
        let mut file_ndx: usize = 0;
        for piece in row_string.chars() {
            if piece.is_ascii_digit() {
                file_ndx += piece.to_digit(10).unwrap_or(0) as usize;
                continue;
            }

            let position = 2_u64.pow((8 * row_index + file_ndx) as u32);
            match piece {
                'p' => chessboard.black_pawns   |= position,
                'r' => chessboard.black_rooks   |= position,
                'b' => chessboard.black_bishops |= position,
                'k' => chessboard.black_king    |= position,
                'q' => chessboard.black_queen   |= position,
                'n' => chessboard.black_knights |= position,
                'P' => chessboard.white_pawns   |= position,
                'R' => chessboard.white_rooks   |= position,
                'B' => chessboard.white_bishops |= position,
                'K' => chessboard.white_king    |= position,
                'Q' => chessboard.white_queen   |= position,
                'N' => chessboard.white_knights |= position,
                //_ => { return Err("Invalid piece in FEN string".to_string()); }
                _ => {},
            }
            file_ndx += 1;
        }
    }
}

/// Validates a Forsyth-Edwards Notation (FEN) string to check if it conforms to the standard format.
///
/// # Arguments
///
/// - `fen`: A FEN string to be validated.
///
/// # Returns
///
/// A boolean indicating whether the FEN string is valid.
#[allow(unused_variables)]
pub fn valid_fen(fen: &str) -> bool {
    let regex = Regex::new(r"^\s*^(((?:[rnbqkpRNBQKP1-8]+\/){7})[rnbqkpRNBQKP1-8]+)\s([b|w])\s(-|[K|Q|k|q]{1,4})\s(-|[a-h][1-8])\s(\d+\s\d+)$").unwrap();

    let captures = regex.captures(fen);
    if captures.is_none() {
        return false;
    }
    let captures = captures.unwrap();

    let fen_ranks = captures.get(1).unwrap().as_str().split('/');
    if fen_ranks.clone().count() != 8 {
        return false;
    }
    for fen_part in fen_ranks {
        let mut piece_count = 0;
        let mut previous_was_digit = false;
        let mut previous_was_piece = false;
        for p in fen_part.chars() {
            if p.is_ascii_digit() {
                if previous_was_digit {
                    return false;
                }
                piece_count += p.to_digit(10).unwrap();
                previous_was_digit = true;
                previous_was_piece = false;
            } else if p == '~' {
                if !previous_was_piece {
                    return false;
                }
                previous_was_digit = false;
                previous_was_piece = false;
            } else if "pnbqkrPBNQKR".contains(p) {
                piece_count += 1;
                previous_was_digit = false;
                previous_was_piece = true;
            } else {
                return false;
            }
        }
        if piece_count != 8 {
            return false;
        }
    }

    let castles = fen.split_whitespace().nth(2).unwrap();
    if !unique_chars(castles) {
        return false;
    }
    true
}

// TODO: Move this somewhere it makes more sense
fn unique_chars(s: &str) -> bool {
    let mut chars = std::collections::HashSet::new();
    for c in s.chars() {
        if !chars.insert(c) {
            return false;
        }
    }
    true
}

/// Generates a Forsyth-Edwards Notation (FEN) string representing the current state of the chessboard.
///
/// # Arguments
///
/// - `chessboard`: A reference to the `Chessboard` struct containing the current game state.
///
/// # Returns
///
/// A string representing the FEN string for the current chessboard position. Example: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
pub fn get_fen_placement(chessboard: &Chessboard) -> String {
    let mut result = String::new();
    for rank in (1..=8).rev() {
        let mut empty_squares = 0;

        for file in 0..=7 {
            let square = position::rank_file_to_square(rank as u8, (file + b'A') as char).unwrap();

            if let Some(p) = chessboard.piece_at_position(square) {
                if empty_squares > 0 {
                    result.push_str(&empty_squares.to_string());
                    empty_squares = 0;
                }
                result.push(p);
            } else {
                empty_squares += 1;
            }
        }
        if empty_squares > 0 {
            result.push_str(&empty_squares.to_string());
        }
        result.push('/');
    }

    result.pop();
    result
}

/// Retrieves the castling rights from the chessboard and returns them in Forsyth–Edwards Notation (FEN) format.
/// 
/// # Arguments
/// 
/// - `chessboard`: A reference to the `Chessboard` struct containing the current game state.
/// 
/// # Returns
/// 
/// A string representing the castling rights in FEN format ("- " if no castling rights).
#[rustfmt::skip]
pub fn get_fen_castles(chessboard: &Chessboard) -> String {
    let state = chessboard.castling_rights;
    let rights: String = ['K', 'Q', 'k', 'q']
        .iter()
        .filter(|&&c| state & match c {
            'K' => 0b1000,
            'Q' => 0b0100,
            'k' => 0b0010,
            'q' => 0b0001,
            _ => 0 // TODO: Throw error here?
        } != 0)
        .collect();

    if rights.is_empty() { "-".to_string() } else { rights }
}

/// Retrieves the en passant square from the chessboard and returns it in algebraic notation.
///
/// # Arguments
///
/// - `chessboard`: A reference to the `Chessboard` struct to get the en passant square from.
///
/// # Returns
///
/// A string representing the en passant square in algebraic notation ("-" if no en passant square).
///
/// If there is no en passant square (0), it returns "-".
pub fn get_fen_passant(chessboard: &Chessboard) -> String {
    let passant = chessboard.en_passant;
    if passant == 0 {
        "-".to_string()
    } else if passant <= 64 && passant > 0 {
        let row = (passant - 1) / 8 + 1;
        let col = (passant - 1) % 8;
        let chr = (b'A' + col) as char;
        format!("{}{}", chr, row)
    } else {
        panic!("En_passant index out of bounds!");
    }
}

/// Parses whose turn it is and updates the chessboard.
///
/// # Arguments
///
/// - `chessboard`: A mutable reference to the `Chessboard` struct to update whose turn it is.
/// - `whose_turn`: A string representing whose turn it is ("w" for white, "b" for black).
pub fn parse_whose_turn(chessboard: &mut Chessboard, whose_turn: &str) {
    chessboard.white_turn = whose_turn == "w";
}

/// Parses castling rights and updates the chessboard.
///
/// # Arguments
///
/// - `chessboard`: A mutable reference to the `Chessboard` struct to update castling rights.
/// - `castle_rights`: A string representing the castling rights part of the FEN string.
pub fn parse_castling_rights(chessboard: &mut Chessboard, castle_rights: &str) {
    chessboard.castling_rights = 0;
    for c in castle_rights.chars() {
        let v = match c {
            'K' => 0b1000,
            'Q' => 0b0100,
            'k' => 0b0010,
            'q' => 0b0001,
            _ => 0b0,
        };
        chessboard.castling_rights |= v;
    }
}

/// Parses the en passant square and updates the chessboard accordingly.
///
/// # Arguments
///
/// - `chessboard`: A mutable reference to the `Chessboard` struct to update the en passant square.
/// - `en_passant`: A string representing the en passant square in algebraic notation (e.g., "e3").
///   If the en passant square is "-" (no en passant square), it is set to 0.
pub fn parse_en_passant(chessboard: &mut Chessboard, en_passant: &str) {
    if let (Some(col), Some(row)) = (
        en_passant.chars().next().map(|c| c.to_ascii_uppercase()),
        en_passant.chars().nth(1).and_then(|c| c.to_digit(10)),
    ) {
        if (1..=8).contains(&row) {
            let col_value: u8 = match col {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                'E' => 5,
                'F' => 6,
                'G' => 7,
                'H' => 8,
                _ => 0,
            };
            chessboard.en_passant = col_value + 8 * (row as u8 - 1);
        }
    } else {
        chessboard.en_passant = 0;
    }
}
