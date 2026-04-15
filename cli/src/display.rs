use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use kno3_chess_engine::position::rank_file_to_square;
use kno3_chess_engine::Chessboard;
use std::io::stdout;

#[rustfmt::skip]
fn find_fg(p: char) -> Color {
    if p.is_uppercase() { Color::White }
    else { Color::Black }
}

#[rustfmt::skip]
fn find_bkgnd(rank: u8, file: u8) -> Color {
    if (rank + file).is_multiple_of(2)
        { Color::Rgb { r: 255, g: 206, b: 158 } }
    else
        { Color::Rgb { r: 190, g: 140, b: 170 } }
}

pub trait DisplayBoard {
    fn display(&self);
}

impl DisplayBoard for Chessboard {
    fn display(&self) {
        let ranks = [8, 7, 6, 5, 4, 3, 2, 1];
        let files = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

        for rank in ranks.iter() {
            print!("{rank} ");
            for (f_index, file) in files.iter().enumerate() {
                let square = rank_file_to_square(*rank, *file).expect("These are manually set");
                let piece = self.piece_at_position(square).unwrap_or('.');

                let fg = find_fg(piece);
                let frmt = format!("{:^3}", piece);
                let bk = find_bkgnd(*rank, f_index as u8);
                let _ = execute!(
                    stdout(),
                    SetForegroundColor(fg),
                    SetBackgroundColor(bk),
                    Print(frmt),
                    ResetColor
                );
            }
            println!();
        }
        print!(" ");
        for file in files.iter() {
            print!(" {file} ");
        }
        println!();
    }
}
