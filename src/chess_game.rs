mod board;
mod board_space;
mod game_piece;
mod new_piece;
mod piece;
mod piece_catalog;

use board::Board;
use piece_catalog::PieceCatalog;

pub struct ChessGame {
    pub board: Board,
    pub piece_catalog: piece_catalog::PieceCatalog,
}

impl ChessGame {
    pub fn new() -> Result<ChessGame, crate::Error> {
        Ok(ChessGame {
            board: Board::new()?,
            piece_catalog: PieceCatalog::new()?,
        })
    }

    pub fn generate_pieces(&mut self) {}
}
