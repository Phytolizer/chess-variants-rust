use super::piece_move::PieceMove;

#[derive(Debug)]
pub struct Piece {
    pub name: String,
    pub image_key: String,
    pub move_set: Vec<PieceMove>,
    pub promotions: Vec<String>,
}

impl Piece {
    pub fn new() -> Self {
        Piece {
            name: "".to_string(),
            image_key: "".to_string(),
            move_set: vec![],
            promotions: vec![],
        }
    }
}
