mod board;
mod board_space;
mod game_piece;
mod new_piece;
mod piece;
mod piece_catalog;

use std::fs;

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

    pub fn load(&mut self) {
        self.piece_catalog.generate("./chess_pieces/".to_string());
        let file = fs::read_dir("./chess_boards/classic_chess".to_string())?;
        self.board.generate(file, &self.piece_catalog);
    }
}
