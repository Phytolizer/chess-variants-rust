use super::piece::Piece;

pub struct PieceCatalog {
    catalog: Vec<Piece>,
}

impl PieceCatalog {
    pub fn new() -> Result<PieceCatalog> {
        Ok(PieceCatalog {
            catalog: Vec![],
        })
    }

    pub fn add_piece(&mut self, piece: Piece) -> String {
        self.catalog.append(piece);
    }
}
