mod board;
mod board_space;
pub(crate) mod chess_textures;
mod game_piece;
mod new_piece;
mod piece;
pub(crate) mod piece_catalog;
mod piece_move;

use std::{fmt::Display, fs};

use board::Board;
use chess_textures::TextureRegistry;
use piece_catalog::PieceCatalog;
use sdl2::render::TextureCreator;

pub struct ChessGame<'tc, C> {
    pub piece_catalog: PieceCatalog,
    pub board: Board,
    pub textures: TextureRegistry<'tc, C>,
}

impl<'tc, C> ChessGame<'tc, C> {
    pub fn new(texture_creator: &'tc TextureCreator<C>) -> Result<ChessGame<'tc, C>, crate::Error> {
        Ok(ChessGame {
            piece_catalog: PieceCatalog::new()?,
            board: Board::new()?,
            textures: TextureRegistry::new(texture_creator),
        })
    }

    pub fn load(&mut self) -> Result<(), crate::Error> {
        self.piece_catalog.generate("./chess_pieces/".to_string())?;
        let mut file = fs::read_dir("./chess_boards/")?;
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

        self.board.generate(board_file, &self.piece_catalog)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct InvalidFormatError {
    line: usize,
    token: String,
}

impl InvalidFormatError {
    pub fn new(line: usize, token: String) -> Self {
        Self { line, token }
    }
}

impl std::error::Error for InvalidFormatError {}

impl Display for InvalidFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid file format (line {}): '{}'",
            self.line, self.token
        )
    }
}
