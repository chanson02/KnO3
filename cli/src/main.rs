use crate::display::DisplayBoard;
mod display;
use clap::{Arg, Command};
use kno3_chess_engine::position;
use kno3_chess_engine::GameState;
use std::fmt;

#[derive(Debug)]
enum Error {
    FENParsingError(String),
    ArgumentError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FENParsingError(msg) => write!(f, "FEN Parsing error: {}", msg),
            Error::ArgumentError(msg) => write!(f, "Argument error: {}", msg),
        }
    }
}

fn main() -> Result<(), Error> {
    let matches = Command::new("KnO3 Chess CLI")
        .version("1.0")
        .about("CLI for interacting with chess games")
        .arg(
            Arg::new("fen")
                .short('f')
                .long("fen")
                .value_name("FEN")
                .help("FEN string representing current game")
                .required(true),
        )
        .arg(
            Arg::new("show")
                .long("show")
                .short('s')
                .help("Prints the state of the board")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("get-moves")
                .long("get-moves")
                .short('g')
                .value_name("POSITION")
                .help("Get possible moves for piece at the given position (ex: 'e2')"),
        )
        .arg(
            Arg::new("evaluate")
                .long("evaluate")
                .short('e')
                .help("Determines who is winning. Positive number indicates a white advantage.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("move")
                .long("move")
                .short('m')
                .value_name("move")
                .help("Move a piece (ex: 'E2:E4'"),
        )
        .get_matches();

    // Happen every time //

    let fen = matches
        .get_one::<String>("fen")
        .ok_or(Error::ArgumentError("FEN string required".to_string()))?;

    let mut gs = GameState::from_string(fen).map_err(|e| Error::FENParsingError(e.to_string()))?;

    // Setters //

    if let Some(move_coords) = matches.get_one::<String>("move") {
        let mut move_coords = move_coords.split(':');
        let start_square = match move_coords.next() {
            Some(square) => square,
            None => return Err(Error::ArgumentError("Invalid move format".to_string())),
        };
        let from = position::string_to_square(start_square)
            .map_err(|e| Error::ArgumentError(e.to_string()))?;
        let end_square = match move_coords.next() {
            Some(square) => square,
            None => return Err(Error::ArgumentError("Invalid move format".to_string())),
        };
        let to = position::string_to_square(end_square)
            .map_err(|e| Error::ArgumentError(e.to_string()))?;
        gs.move_piece_legally(from, to)
            .map_err(|e| Error::ArgumentError(e.to_string()))?;
    }

    // Getters //

    if matches.get_flag("show") {
        gs.board.display();
    }
    if matches.get_flag("evaluate") {
        println!("{}", gs.board.evaluate());
    }
    if let Some(position) = matches.get_one::<String>("get-moves") {
        let square = position::string_to_square(position)
            .map_err(|e| Error::ArgumentError(e.to_string()))?;
        let moves = position::active_squares(gs.possible_moves(square))
            .into_iter()
            .map(position::square_to_string)
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", moves);
    }

    Ok(())
}
