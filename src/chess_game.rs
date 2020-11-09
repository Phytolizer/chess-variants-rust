pub mod board;

pub use board::Board;
pub use piece_catalog::PieceCatalog;

pub struct ChessGame {
    pub board: Board,
    pub piece_catalog: PieceCatalog,
}

impl ChessGame {
    pub fn new() -> Result<ChessGame> {
        Ok(ChessGame {
            board: Board::new(),
            piece_catalog: PieceCatalog::new(),
        })
    }

    pub fn generate_pieces(&mut self) {
        match self.settings.game_type {
            //GameType::Classic => generate_classic(&mut self.settings),
            GameType::Random => self.generate_random(),
            //_ => return,
        }
    }

    pub fn display_pieces<RT>(&self, canvas: &mut Canvas<RT>) -> Result<(), crate::Error>
    where
        RT: RenderTarget,
    {
        self.pieces
            .iter()
            .map(|p| p.display(canvas))
            .collect::<Result<_, _>>()?;
        Ok(())
    }
    //pub fn generate_classic(settings: &mut ChessSettings) {}
    pub fn generate_random(&mut self) {
        self.pieces.clear();
        let mut rng = rand::thread_rng();
        for row in 0..self.settings.starting_rows {
            for col in 0..self.settings.squares_horz {
                let index = rng.gen_range(0, self.settings.factory.len());
                self.pieces.push(self.settings.factory[index].build_piece(
                    0,
                    self.settings.squares_vert - row - 1,
                    col,
                ));
                self.pieces
                    .push(self.settings.factory[index].build_piece(1, row, col));
            }
        }
    }
}
