use super::{piece, position, Chessboard};

impl Chessboard {
    /// Maps the pieces on the chessboard to their character representations in the console.
    ///
    /// # Returns
    ///
    /// An array of tuples, where each tuple consists of a chess piece character and its
    /// corresponding bitboard positions. The characters represent different chess pieces,
    /// and the bitboard positions indicate the squares occupied by those pieces on the board.
    ///
    /// The function returns a vector containing tuples, each associating a chess piece
    /// character ('P', 'N', 'B', 'K', 'Q', 'R', 'p', 'n', 'b', 'k', 'q', 'r') with its
    /// corresponding bitboard positions on the chessboard.
    //fn get_pieces(&self) -> Vec<(char, u64)> {
    pub(crate) fn get_pieces(&self) -> [(char, u64); 12] {
        [
            ('P', self.white_pawns),
            ('N', self.white_knights),
            ('B', self.white_bishops),
            ('K', self.white_king),
            ('Q', self.white_queen),
            ('R', self.white_rooks),
            ('p', self.black_pawns),
            ('n', self.black_knights),
            ('b', self.black_bishops),
            ('k', self.black_king),
            ('q', self.black_queen),
            ('r', self.black_rooks),
        ]
    }

    pub fn one_side_pieces(&self, white: bool) -> u64 {
        if white {
            self.white_bishops
                & self.white_king
                & self.white_knights
                & self.white_pawns
                & self.white_rooks
                & self.white_queen
        } else {
            self.black_bishops
                & self.black_king
                & self.black_knights
                & self.black_pawns
                & self.black_rooks
                & self.black_queen
        }
    }

    pub fn both_side_pieces(&self) -> u64 {
        self.one_side_pieces(true) | self.one_side_pieces(false)
    }

    /// Retrieves the chess piece at a specific position on the chessboard.
    ///
    /// # Arguments
    ///
    /// - `square`: The square number (0 = bottom left, 63 = top right)
    ///
    /// # Returns
    ///
    /// The character representation of the piece at the specified position.
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::{Chessboard, position};
    /// let initial_position = Chessboard::new();
    /// let square = position::rank_file_to_square(1, 'A').unwrap();
    /// let piece_at_a1 = initial_position.piece_at_position(square).unwrap_or('.');
    /// println!("Piece at a1: {}", piece_at_a1);
    /// ```
    /// Note: Uppercase pieces are white and lowercase pieces are black.
    pub fn piece_at_position(&self, square: i64) -> Option<char> {
        let btwise = 1 << square;
        for (p_type, positions) in self.get_pieces() {
            if btwise & positions != 0 {
                return Some(p_type);
            }
        }
        None
    }

    /// Determine if this piece is legally able to move from cur_square to new_square.
    ///
    /// # Arguments
    ///
    /// - `piece`: The character representation of the piece. Uppercase is white.
    /// - `cur_square`: square number (0 is bottom left, 63 is top right) where the piece currently is.
    /// - `new_square`: square number (0 is bottom left, 63 is top right) where the piece is trying to move to.
    ///
    /// # Returns.
    ///
    /// If the piece is legally allowed to move
    pub fn is_valid_move_for_piece(piece: char, cur_square: i64, new_square: i64) -> bool {
        if cur_square == new_square {
            return false;
        } // cannot move onto itself
        if cur_square > 63 || new_square > 63 {
            return false;
        } // bigger than the board
        match piece {
            'p' => piece::legal_pawn(false, cur_square, new_square),
            'P' => piece::legal_pawn(true, cur_square, new_square),
            'r' => piece::legal_rook(cur_square, new_square),
            'R' => piece::legal_rook(cur_square, new_square),
            'b' => piece::legal_bishop(cur_square, new_square),
            'B' => piece::legal_bishop(cur_square, new_square),
            'k' => piece::legal_king(cur_square, new_square),
            'K' => piece::legal_king(cur_square, new_square),
            'q' => piece::legal_queen(cur_square, new_square),
            'Q' => piece::legal_queen(cur_square, new_square),
            'n' => piece::legal_knight(cur_square, new_square),
            'N' => piece::legal_knight(cur_square, new_square),
            _ => false,
        }
    }

    /// Get all legal moves this piece is legally able to make
    ///
    /// # Arguments
    ///
    /// - `square`: The square number where the piece looking to move is
    ///
    /// # Returns
    ///
    /// A vector of square positions
    ///
    /// # Panics
    ///
    /// Panics if provided a square out of bounds or does not contain a piece
    pub fn get_legal_moves(&self, square: i64) -> Vec<i64> {
        let piece = self
            .piece_at_position(square)
            .expect("No piece at this position");
        match piece {
            'p' => self.get_pawn_moves(square, false),
            'P' => self.get_pawn_moves(square, true),
            _ => Vec::new(),
        }
    }

    /// Moves a chess piece on the chessboard from the current position to the new position.
    ///
    /// # Arguments
    ///
    /// * `current_pos` - The current position of the piece in algebraic notation (e.g., "E2").
    /// * `new_pos` - The new position to move the piece to in algebraic notation (e.g., "E4").
    /// * `piece` - The type of chess piece to be moved (e.g., 'p' for pawn, 'R' for rook).
    ///
    /// # Example
    ///
    /// ```
    /// use kn_o3::board::Chessboard;
    /// let mut chessboard = Chessboard::new();
    /// chessboard.move_piece("E2", "E4", 'P');
    /// ```
    pub fn move_piece(&mut self, current_pos: &str, new_pos: &str, piece: char) {
        let old_square = match position::string_to_square(current_pos) {
            Ok(square) => square,
            _ => return,
        };
        let new_square = match position::string_to_square(new_pos) {
            Ok(square) => square,
            _ => return,
        };

        let two: u64 = 2;
        let clear_old = !two.pow(old_square.try_into().unwrap());
        let add_new = two.pow(new_square.try_into().unwrap());

        // Delete the piece from the old square
        match piece {
            'p' => {
                self.black_pawns &= clear_old; // Clear old position
                self.black_pawns |= add_new; // Set new position
            }
            'r' => {
                self.black_rooks &= clear_old;
                self.black_rooks |= add_new;
            }
            'b' => {
                self.black_bishops &= clear_old;
                self.black_bishops |= add_new;
            }
            'k' => {
                self.black_king &= clear_old;
                self.black_king |= add_new;
            }
            'q' => {
                self.black_queen &= clear_old;
                self.black_queen |= add_new;
            }
            'n' => {
                self.black_knights &= clear_old;
                self.black_knights |= add_new;
            }
            'P' => {
                self.white_pawns &= clear_old;
                self.white_pawns |= add_new;
            }
            'R' => {
                self.white_rooks &= clear_old;
                self.white_rooks |= add_new;
            }
            'B' => {
                self.white_bishops &= clear_old;
                self.white_bishops |= add_new;
            }
            'K' => {
                self.white_king &= clear_old;
                self.white_king |= add_new;
            }
            'Q' => {
                self.white_queen &= clear_old;
                self.white_queen |= add_new;
            }
            'N' => {
                self.white_knights &= clear_old;
                self.white_knights |= add_new;
            }
            _ => {}
        }
    }
}
