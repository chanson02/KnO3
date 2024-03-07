mod board;
mod fen_util;
use crate::board::{position, Chessboard};

fn main() {
    let mut cb = Chessboard::new();
    println!("{:?}", cb);
    println!("{cb}\n\n");

    // Move a white pawn from E2 to E3
    cb.move_piece("E2", "E3", 'P');
    println!("{cb}\n\n");

    // Move a black knight from G8 to F6
    cb.move_piece("G8", "F6", 'n');
    println!("{cb}\n\n");

    // Move a white queen from D1 to H5
    cb.move_piece("D1", "H5", 'Q');
    println!("{cb}\n\n");

    let x = position::square_to_rank_file(22);
    println!("{} {}", x.0, x.1);

    cb = Chessboard::from_string("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 1 2")
        .unwrap();
    println!("{cb}");

    test_valid_move_for_piece('P', "E4", "E4");
    test_valid_move_for_piece('K', "E1", "E1");
    test_valid_move_for_piece('N', "F3", "G4");
    test_valid_move_for_piece('B', "F1", "G7");
    test_valid_move_for_piece('R', "I1", "G2");

    Chessboard::both_side_pieces(&cb);
    Chessboard::one_side_pieces(&cb, true);
}

fn test_valid_move_for_piece(p: char, cur_coord: &str, new_coord: &str) {
    println!("Attempting to move {p} from {cur_coord} to {new_coord}");
    let cur_square = match position::string_to_square(cur_coord) {
        Ok(square) => square,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let new_square = match position::string_to_square(new_coord) {
        Ok(square) => square,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let legal = Chessboard::is_valid_move_for_piece(p, cur_square, new_square);
    println!("legal? {legal}");
}

// //min function
// #[rustfmt::skip]
// fn min<T: Ord>(a: T, b: T) -> T {
//     if a < b { a } else { b }
// }
// //max function
// #[rustfmt::skip]
// fn max<T: Ord>(a: T, b: T) -> T {
//     if a > b { a } else { b }
// }
