/// A square is a 0-63 number
pub fn square_to_rank(square: u8) -> u8 {
    (square / 8) + 1
}

pub fn square_to_file(square: u8) -> char {
    ((square % 8) + b'A') as char
}

pub fn square_to_rank_file(square: u8) -> (u8, char) {
    let rank = square_to_rank(square);
    let file = square_to_file(square);
    (rank, file)
}

#[rustfmt::skip]
pub fn rank_file_to_square(rank: u8, mut file: char) -> Result<u8, String> {
    file.make_ascii_uppercase();
    if !(1..=8).contains(&rank)     { return Err(format!("Invalid rank {rank}")); }
    if !('A'..='H').contains(&file) { return Err(format!("Invalid file {file}")); }
    Ok((rank - 1) * 8 + (file as u8 - b'A'))
}

pub fn square_to_string(square: u8) -> String {
    let (rank, file) = square_to_rank_file(square);
    format!("{}{}", file, rank)
}

/// Coordinate string `A1` - `H8`
pub fn string_to_square(coord: &str) -> Result<u8, String> {
    let chars: Vec<char> = coord.chars().collect();
    if chars.len() != 2 {
        return Err(format!("Invalid coordinate input: {coord}"));
    }

    let rank = chars[1]
        .to_digit(10)
        .ok_or_else(|| format!("Invalid rank: {}", chars[1]))? as u8;
    let file = chars[0];
    rank_file_to_square(rank, file)
}

/// Find the squares turned on in this bitboard
pub fn active_squares(mut bitboard: u64) -> Vec<u8> {
    let mut squares = Vec::new();
    let mut index = 0;

    while bitboard != 0 {
        if bitboard & 1 != 0 {
            squares.push(index);
        }
        index += 1;
        bitboard >>= 1;
    }
    squares
}

#[rustfmt::skip]
const PAWN_EVALUATION_TABLE: [f64; 64] = [
    -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67,
    -0.50, -0.33, -0.33, -1.00, -1.00, -0.33, -0.33, -0.50,
    -0.50, -0.83, -1.00, -0.67, -0.67, -1.00, -0.83, -0.50,
    -0.67, -0.67, -0.67,  0.17,  0.17, -0.67, -0.67, -0.67,
    -0.50, -0.50, -0.33,  0.33,  0.33, -0.33, -0.50, -0.50,
    -0.33, -0.33,  0.00,  0.33,  0.33,  0.00, -0.33, -0.33,
     1.00,  1.00,  1.00,  1.00,  1.00,  1.00,  1.00,  1.00,
    -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67, -0.67,
];

#[rustfmt::skip]
const KNIGHT_EVALUATION_TABLE: [f64; 64] = [
    -1.00, -0.73, -0.47, -0.47, -0.47, -0.47, -0.73, -1.00,
    -0.73, -0.20,  0.33,  0.47,  0.47,  0.33, -0.20, -0.73,
    -0.47,  0.33,  0.73,  0.87,  0.87,  0.73,  0.33, -0.47,
    -0.47,  0.47,  0.87,  1.00,  1.00,  0.87,  0.47, -0.47,
    -0.47,  0.33,  0.87,  1.00,  1.00,  0.87,  0.33, -0.47,
    -0.47,  0.47,  0.73,  0.87,  0.87,  0.73,  0.47, -0.47,
    -0.73, -0.20,  0.33,  0.47,  0.47,  0.33, -0.20, -0.73,
    -1.00, -0.73, -0.47, -0.47, -0.47, -0.47, -0.73, -1.00,
];

#[rustfmt::skip]
const BISHOP_EVALUATION_TABLE: [f64; 64] = [
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
    -0.50,  0.00,  0.25,  0.25,  0.25,  0.25,  0.00, -0.50,
    -0.50,  0.25,  0.50,  0.75,  0.75,  0.50,  0.25, -0.50,
    -0.50,  0.25,  0.75,  1.00,  1.00,  0.75,  0.25, -0.50,
    -0.50,  0.00,  0.75,  1.00,  1.00,  0.75,  0.00, -0.50,
    -0.50,  0.50,  0.75,  0.75,  0.75,  0.75,  0.50, -0.50,
    -0.50,  0.25,  0.00,  0.00,  0.00,  0.00,  0.25, -0.50,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
];

#[rustfmt::skip]
const ROOK_EVALUATION_TABLE: [f64; 64] = [
    -0.50, -0.50, -0.50,  0.00,  0.00, -0.50, -0.50, -0.50,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
    -1.00, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -1.00,
     0.00,  1.00,  1.00,  1.00,  1.00,  1.00,  1.00,  0.00,
    -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50, -0.50,
];

#[rustfmt::skip]
const QUEEN_EVALUATION_TABLE: [f64; 64] = [
    -1.00, -0.33, -0.33,  0.00,  0.00, -0.33, -0.33, -1.00,
    -0.33,  0.33,  0.67,  0.67,  0.67,  0.67,  0.33, -0.33,
    -0.33,  0.33,  0.83,  0.83,  0.83,  0.83,  0.33, -0.33,
     0.00,  0.33,  0.83,  1.00,  1.00,  0.83,  0.33,  0.00,
     0.33,  0.33,  0.83,  1.00,  1.00,  0.83,  0.33,  0.33,
    -0.33,  0.67,  0.83,  0.83,  0.83,  0.83,  0.33, -0.33,
    -0.33,  0.33,  0.67,  0.33,  0.33,  0.33,  0.33, -0.33,
    -1.00, -0.33, -0.33,  0.00,  0.00, -0.33, -0.33, -1.00,
];

#[rustfmt::skip]
const KING_EVALUATION_TABLE: [f64; 64] = [
    -0.53, -0.76, -0.76, -1.00, -1.00, -0.76, -0.76, -0.53,
    -0.53, -0.76, -0.76, -1.00, -1.00, -0.76, -0.76, -0.53,
    -0.53, -0.76, -0.76, -1.00, -1.00, -0.76, -0.76, -0.53,
    -0.53, -0.76, -0.76, -1.00, -1.00, -0.76, -0.76, -0.53,
    -0.29, -0.53, -0.53, -0.76, -0.76, -0.53, -0.53, -0.29,
    -0.06, -0.29, -0.29, -0.41, -0.41, -0.29, -0.29, -0.06,
     0.76,  0.76,  0.18,  0.06,  0.06,  0.18,  0.76,  0.76,
     0.65,  1.00,  0.53,  0.18,  0.18,  0.53,  1.00,  0.65,
];

/// Returns the evaluation table for a given piece type.
/// The input should be an ASCII character representing the piece ('P', 'N', 'B', 'R', 'Q', 'K').
pub fn get_piece_evaluation_table(piece: char) -> Option<&'static [f64; 64]> {
    match piece.to_ascii_uppercase() {
        'P' => Some(&PAWN_EVALUATION_TABLE),
        'N' => Some(&KNIGHT_EVALUATION_TABLE),
        'B' => Some(&BISHOP_EVALUATION_TABLE),
        'R' => Some(&ROOK_EVALUATION_TABLE),
        'Q' => Some(&QUEEN_EVALUATION_TABLE),
        'K' => Some(&KING_EVALUATION_TABLE),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_to_rank() {
        assert_eq!(square_to_rank(0), 1);
        assert_eq!(square_to_rank(7), 1);
        assert_eq!(square_to_rank(8), 2);
        assert_eq!(square_to_rank(63), 8);
    }

    #[test]
    fn test_square_to_file() {
        assert_eq!(square_to_file(0), 'A');
        assert_eq!(square_to_file(7), 'H');
        assert_eq!(square_to_file(8), 'A');
        assert_eq!(square_to_file(62), 'G')
    }

    #[test]
    fn test_to_square() {
        assert_eq!(rank_file_to_square(1, 'A').unwrap(), 0);
        assert_eq!(rank_file_to_square(1, 'H').unwrap(), 7);
        assert_eq!(rank_file_to_square(2, 'A').unwrap(), 8);
        assert_eq!(rank_file_to_square(8, 'H').unwrap(), 63);
        assert_eq!(
            rank_file_to_square(1, 'B').unwrap(),
            rank_file_to_square(1, 'b').unwrap()
        );

        assert_eq!(string_to_square("A1").unwrap(), 0);
        assert_eq!(string_to_square("H1").unwrap(), 7);
        assert_eq!(string_to_square("A2").unwrap(), 8);
        assert_eq!(string_to_square("H8").unwrap(), 63);
        assert_eq!(
            string_to_square("B1").unwrap(),
            string_to_square("b1").unwrap()
        );

        assert!(rank_file_to_square(0, 'A').is_err());
        assert!(rank_file_to_square(1, 'I').is_err());
        assert!(string_to_square("A0").is_err());
        assert!(string_to_square("I1").is_err());
    }

    #[test]
    fn test_acitve_squares() {
        assert_eq!(active_squares(0b10101010), vec![1, 3, 5, 7]);
        assert_eq!(active_squares(0b11100000), vec![5, 6, 7]);
        assert_eq!(active_squares(0), vec![]);
    }
}
