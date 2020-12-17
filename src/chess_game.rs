mod board;
mod board_space;
mod game_piece;
mod piece;
pub(crate) mod piece_catalog;
mod piece_move;
pub(crate) mod texture_registry;

use parking_lot::RwLock;
use sdl2::render::{TextureCreator, WindowCanvas};
use std::{fmt::Display, fs, rc::Rc};

pub struct ChessGame<'tc, C> {
    pub piece_catalog: piece_catalog::PieceCatalog,
    pub board: board::Board,
    pub textures: texture_registry::TextureRegistry<'tc, C>,
}

impl<'tc, C> ChessGame<'tc, C> {
    pub fn new(texture_creator: &'tc TextureCreator<C>) -> Result<ChessGame<'tc, C>, crate::Error> {
        Ok(ChessGame {
            piece_catalog: piece_catalog::PieceCatalog::new()?,
            board: board::Board::new()?,
            textures: texture_registry::TextureRegistry::new(texture_creator),
        })
    }

    pub fn load(&mut self) -> Result<(), crate::Error> {
        self.piece_catalog.generate("./chess_pieces/".to_string())?;
        self.textures
            .generate_piece_images("./chess_images".to_string())?;
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

    pub fn render_board(
        &mut self,
        canvas: Rc<RwLock<WindowCanvas>>,
        width: u32,
        height: u32,
    ) -> Result<(), crate::Error> {
        self.textures
            .render_board(canvas, (width, height), &mut self.board)?;
        Ok(())
    }

    pub fn mouse_hover(&mut self, x: &i32, y: &i32) -> Result<(), crate::Error> {
        self.board.mouse_hover(x, y)
    }

    pub fn mouse_left_click(&mut self) -> Result<(), crate::Error> {
        self.board.mouse_left_click(&self.piece_catalog)
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
