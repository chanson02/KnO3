use crate::position::rank_file_to_square;
use std::cmp::PartialEq;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Chessboard {
    pub black_pawns: u64,
    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queen: u64,
    pub black_king: u64,
    pub white_pawns: u64,
    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queen: u64,
    pub white_king: u64,
}

impl Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fen = String::new();

        for rank in (1..=8).rev() {
            let mut blank = 0;
            for file in 'A'..='H' {
                if let Some(piece) = self.piece_at_position(
                    rank_file_to_square(rank, file).expect("Expected rank 1-8 file A-H"),
                ) {
                    if blank > 0 {
                        fen.push_str(&blank.to_string());
                        blank = 0;
                    }
                    fen.push(piece);
                } else {
                    blank += 1;
                }
            }
            if blank > 0 {
                fen.push_str(&blank.to_string());
            }
            if rank > 1 {
                fen.push('/');
            }
        }
        write!(f, "{}", fen)
    }
}

impl Chessboard {
    /// piece placement portion of the FEN string
    pub fn from_string(piece_placement: &str) -> Result<Self, String> {
        let mut result = Chessboard::empty();

        let mut rank_ndx = 8;
        for pieces in piece_placement.split('/') {
            let mut file_ndx = b'A';

            for piece in pieces.chars() {
                if piece.is_ascii_digit() {
                    file_ndx += piece.to_digit(10).expect("Already checked") as u8;
                    continue;
                }

                let square = rank_file_to_square(rank_ndx, file_ndx as char)?;
                *result.piece_bitboard(piece)? |= 1 << square;
                file_ndx += 1;
            }
            rank_ndx -= 1;
        }

        Ok(result)
    }

    pub fn new() -> Chessboard {
        let pawns = 0xFF;
        let rooks = 0x81;
        let knights = 0x42;
        let bishops = 0x24;
        let queen = 0x08;
        let king = 0x10;

        let top_row = 56; // 7 rows * 8 bits

        Chessboard {
            white_rooks: rooks,
            white_knights: knights,
            white_bishops: bishops,
            white_pawns: pawns << 8,
            white_queen: queen,
            white_king: king,

            black_rooks: rooks << top_row,
            black_knights: knights << top_row,
            black_bishops: bishops << top_row,
            black_queen: queen << top_row,
            black_king: king << top_row,
            black_pawns: pawns << (top_row - 8),
        }
    }

    pub fn empty() -> Self {
        Self {
            white_rooks: 0,
            white_knights: 0,
            white_bishops: 0,
            white_pawns: 0,
            white_queen: 0,
            white_king: 0,

            black_rooks: 0,
            black_knights: 0,
            black_bishops: 0,
            black_queen: 0,
            black_king: 0,
            black_pawns: 0,
        }
    }
}

impl PartialEq for Chessboard {
    /// Would've been easier to compare FEN's here, but this is faster
    fn eq(&self, other: &Self) -> bool {
        // Compare pawns first because they're most likely to have been moved
        self.black_pawns == other.black_pawns
            && self.white_pawns == other.white_pawns
            && self.black_rooks == other.black_rooks
            && self.white_rooks == other.white_rooks
            && self.black_knights == other.black_knights
            && self.white_knights == other.white_knights
            && self.white_bishops == other.white_bishops
            && self.black_bishops == other.black_bishops
            && self.white_queen == other.white_queen
            && self.black_queen == other.black_queen
            && self.white_king == other.white_king
            && self.black_king == other.black_king
    }
}

impl fmt::Debug for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_placement() {
        let placement = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board = Chessboard::from_string(placement).unwrap();
        assert_eq!(board, Chessboard::new());
        assert_eq!(board.to_string(), placement);
    }
}
