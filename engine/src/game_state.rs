use core::fmt;
use std::fmt::Display;

use crate::position;
use crate::Chessboard;

// https://www.chess.com/terms/fen-chess
#[derive(Clone)]
pub struct GameState {
    pub white_turn: bool,
    pub castling: u8,   // KQkq will be represented by 4 bits
    pub en_passant: u8, // a square that has en passant ability
    half_clock: u32,
    move_count: u32,
    pub board: Chessboard,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for GameState {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = if self.white_turn { 'w' } else { 'b' };

        let mut castles = String::new();
        if self.castling & 0b1000 != 0 { castles.push('K'); }
        if self.castling & 0b0100 != 0 { castles.push('Q'); }
        if self.castling & 0b0010 != 0 { castles.push('k'); }
        if self.castling & 0b0001 != 0 { castles.push('q'); }
        if castles.is_empty() { castles.push('-'); }

        let en_passant = if self.en_passant > 63 { "-".to_string() } else {
            format!("{}{}", position::square_to_file(self.en_passant), position::square_to_rank(self.en_passant))
        };

        write!(
            f,
            "{} {} {} {} {} {}",
            self.board, color, castles, en_passant, self.half_clock, self.move_count
        )
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            white_turn: true,
            castling: 0x0F,
            en_passant: 255,
            half_clock: 0,
            move_count: 1,
            board: Chessboard::new(),
        }
    }

    pub fn from_string(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Invalid FEN string".to_string());
        }

        let passant = match parts[3] {
            "-" => 255, // out of range
            _ => position::string_to_square(parts[3])
                .map_err(|e| format!("Invalid en passant: {e}"))?,
        };

        Ok(Self {
            white_turn: parts[1] == "w",
            castling: parse_castling_rights(parts[2]),
            en_passant: passant,
            half_clock: parts[4]
                .parse()
                .map_err(|_| "Invalid half clock".to_string())?,
            move_count: parts[5]
                .parse()
                .map_err(|_| "Invalid move count".to_string())?,
            board: Chessboard::from_string(parts[0])?,
        })
    }
}

/// part: The portion of the fen string that marks castling
fn parse_castling_rights(part: &str) -> u8 {
    let mut result = 0;
    for c in part.chars() {
        let v = match c {
            'K' => 0b1000,
            'Q' => 0b0100,
            'k' => 0b0010,
            'q' => 0b0001,
            _ => 0,
        };
        result |= v;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen = GameState::from_string(fen_str).unwrap();

        assert!(fen.white_turn);
        assert_eq!(fen.castling, 0b1111);
        assert_eq!(fen.en_passant, 255);
        assert_eq!(fen.half_clock, 0);
        assert_eq!(fen.move_count, 1);
    }

    #[test]
    fn test_passant() {
        let mut fen =
            GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(fen.unwrap().en_passant, 255);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1");
        assert_eq!(fen.unwrap().en_passant, 20);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq i3 0 1");
        assert!(fen.is_err());
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e9 0 1");
        assert!(fen.is_err());
    }

    #[test]
    fn test_castles() {
        let mut fen =
            GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        assert_eq!(fen.castling, 0b1111);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Kk - 0 1")
            .unwrap();
        assert_eq!(fen.castling, 0b1010);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qq - 0 1")
            .unwrap();
        assert_eq!(fen.castling, 0b0101);
        fen = GameState::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1")
            .unwrap();
        assert_eq!(fen.castling, 0);
    }

    #[test]
    fn test_invalid_fen() {
        let mut result = GameState::from_string("invalid fen string");
        assert!(result.is_err());

        result = GameState::from_string("positions turn castles passant clock move");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_string() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(GameState::from_string(fen).expect("").to_string(), fen);
    }
}
