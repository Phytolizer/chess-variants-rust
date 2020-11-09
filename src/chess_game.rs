mod board;
mod board_space;
mod chess_textures;
mod game_piece;
mod new_piece;
mod piece;
pub(crate) mod piece_catalog;
mod piece_move;

use std::{fmt::Display, fs};

use board::Board;
use piece_catalog::PieceCatalog;

pub struct ChessGame {
    pub piece_catalog: PieceCatalog,
    pub board: Board,
}

impl ChessGame {
    pub fn new() -> Result<ChessGame, crate::Error> {
        Ok(ChessGame {
            piece_catalog: PieceCatalog::new()?,
            board: Board::new()?,
        })
    }

    pub fn load(&mut self) -> Result<(), crate::Error> {
        self.piece_catalog.generate("./chess_pieces/".to_string());
        let file = fs::read_dir("./chess_boards/")?;
        // FIXME let user pick a board
        let board_file = file
            .find_map(|f| {
                if let Ok(de) = f {
                    if de.file_name() == "classic_chess.txt" {
                        return Some(de);
                    }
                }
                None
            })
            .unwrap();

        self.board.generate(board_file, &self.piece_catalog);
        Ok(())
    }
}

#[derive(Debug)]
pub struct InvalidFormatError {
    line: usize,
}

impl InvalidFormatError {
    pub fn new(line: usize) -> Self {
        Self { line }
    }
}

impl std::error::Error for InvalidFormatError {}

impl Display for InvalidFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid file format (line {})", self.line)
    }
}
