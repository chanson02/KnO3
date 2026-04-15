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

    match matches.get_one::<String>("move") {
        None => (),
        Some(coords) => move_piece(coords, &mut gs).map_err(Error::ArgumentError)?,
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

fn move_piece(move_string: &str, game: &mut GameState) -> Result<(), String> {
    let mut coords = move_string.split(':');

    let from = match coords.next() {
        None => return Err("Invalid move format. Start position not supplied".to_string()),
        Some(coord) => position::string_to_square(coord)?,
    };

    let to = match coords.next() {
        None => return Err("Invalid move format. End position not supplied".to_string()),
        Some(coord) => position::string_to_square(coord)?,
    };

    game.move_piece_legally(from, to)
}
