pub struct Piece {
    pub piece_name: String,
    pub image_path: String,
    pub move_set: Vec<Vec<bool>>,
    pub kill_set: Vec<Vec<bool>>,
}

impl Piece {
    pub fn new(
        name: String,
        image: String,
        moves: Vec<Vec<bool>>,
        kills: Vec<Vec<bool>>,
    ) -> Result<Piece, crate::Error> {
        Ok(Piece {
            piece_name: name,
            image_path: image,
            move_set: moves,
            kill_set: kills,
        })
    }
}