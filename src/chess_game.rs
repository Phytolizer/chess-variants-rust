mod board;
mod board_space;
mod game_piece;
mod new_piece;
mod piece;
mod piece_catalog;
mod piece_factory;

pub struct ChessGame {
    pub board: board::Board,
    pub piece_catalog: piece_catalog::PieceCatalog,
}

impl ChessGame {
    pub fn new() -> Result<ChessGame, crate::Error> {
        Ok(ChessGame {
            board: board::Board::new(),
            piece_catalog: piece_catalog::PieceCatalog::new(),
        })
    }

    pub fn generate_pieces(&mut self) {
        
    }
}
