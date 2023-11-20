use regex::Regex;

use crate::board::Chessboard;

/// Places pieces on the chessboard based on the FEN (Forsyth-Edwards Notation) rows provided.
/// 
/// # Arguments
/// 
/// * `chessboard` - A mutable reference to the chessboard.
/// * `fen_rows` - FEN string representing the piece placement on the board.
#[rustfmt::skip]
pub fn place_pieces(chessboard: &mut Chessboard, fen_rows: &str) {
    for (row_index, row_string) in fen_rows.split('/').rev().enumerate() {
        let mut file_ndx: usize = 0;
        for piece in row_string.chars() {
            if piece.is_digit(10) {
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

/// Function to check if a given FEN (Forsyth-Edwards Notation) string is valid
///
/// # Arguments
///
/// * `fen` - A FEN string to be validated.
///
/// # Returns
///
/// A boolean indicating whether the FEN string is valid.
pub fn valid_fen(fen: &str) -> bool {
    // Define the regex pattern for a valid FEN string
    let regex = Regex::new(r"^\s*^(((?:[rnbqkpRNBQKP1-8]+\/){7})[rnbqkpRNBQKP1-8]+)\s([b|w])\s([K|Q|k|q]{1,4})\s(-|[a-h][1-8])\s(\d+\s\d+)$").unwrap();
    // Check if the provided FEN string matches the pattern
    if let Some(captures) = regex.captures(fen) {
        // Extract the position part of the FEN and split it into rows
        let fen_list = captures.get(1).unwrap().as_str().split("/");
        // Check if there are exactly 8 rows in the position part
        if fen_list.clone().count() != 8 {
            return false;
        }
        // Iterate through each row in the position part
        for fen_part in fen_list {
            let mut field_sum = 0;
            let mut previous_was_digit = false;
            let mut previous_was_piece = false;
            // Iterate through each character in the row
            for c in fen_part.chars() {
                // Check if the character is a digit
                if c.is_digit(10) {
                    // Check for two subsequent digits
                    if previous_was_digit {
                        return false;
                    }
                    // Accumulate the digit to the column sum
                    field_sum += c.to_digit(10).unwrap();
                    previous_was_digit = true;
                    previous_was_piece = false;
                } else if c == '~' {
                    // Check if ~ is not after a piece
                    if !previous_was_piece {
                        return false;
                    }
                    previous_was_digit = false;
                    previous_was_piece = false;
                } else if "pnbqkrPBNQKR".contains(c) {
                    // Count the piece and update flags
                    field_sum += 1;
                    previous_was_digit = false;
                    previous_was_piece = true;
                } else {
                    // Invalid character in the FEN string
                    return false;
                }
            }
            // Check if there are exactly 8 columns in each row
            if field_sum != 8 {
                return false;
            }
        }
        // The FEN string passed all checks and is valid
        true
    } else {
        // The FEN string does not match the expected pattern
        false
    }
}

/// Generates a FEN (Forsyth-Edwards Notation) string representing the current state of the chessboard.
pub fn get_fen_placement(chessboard: &Chessboard) -> String {
    let mut result: String = "".to_string();
    for rank in (1..=8).rev() {
        let mut empty_squares = 0;

        for file in 0..=7 {
            let p = chessboard.piece_at_position(rank, file);
            if p == '.' {
                empty_squares += 1;
                continue;
            } else if empty_squares > 0 {
                result.push_str(&empty_squares.to_string());
                empty_squares = 0;
            }
            result.push_str(&p.to_string());
        }
        if empty_squares > 0 {
            result.push_str(&empty_squares.to_string());
        }
        result.push_str(&"/".to_string());
    }

    let mut c = result.chars();
    c.next_back();
    c.as_str().to_string()
}

/// Get the castling right substring of a FEN
/// * `chessboard` - The chessboard containing the current game state
/// # Returns
/// A string representing the caslting rights in FEN format
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

pub fn get_fen_passant(chessboard: &Chessboard) -> String {
    let passant = chessboard.en_passant;
    if passant == 0 {
        return "-".to_string();
    }
    let row = (passant - 1) / 8 + 1;
    let col = (passant - 1) % 8;
    let chr = (b'A' + col) as char;
    format!("{}{}", chr, row)
}

/// Parse the piece placement part of the FEN string.
pub fn parse_piece_placement(
    chessboard: &mut Chessboard,
    piece_placement: &str,
) -> Result<(), String> {
    self::place_pieces(chessboard, piece_placement);
    Ok(())
}

/// Parse whose turn it is.
pub fn parse_whose_turn(chessboard: &mut Chessboard, whose_turn: &str) {
    chessboard.white_turn = whose_turn == "w";
}

/// Parse castling rights.
pub fn parse_castling_rights(chessboard: &mut Chessboard, castle_rights: &str) {
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

/// Parse en passant square.
pub fn parse_en_passant(chessboard: &mut Chessboard, en_passant: &str) {
    if en_passant != "-" {
        if let (Some(col), Some(row)) = (
            en_passant.chars().nth(0).map(|c| c.to_ascii_uppercase()),
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
        }
    }
}