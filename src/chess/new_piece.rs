pub struct Piece {
    piece_name: String,
    image_path: String,
    move_set: Vec<Vec<bool>>,
    kill_set: Vec<Vec<bool>>,
}

impl Piece {
    pub fn new(
        name: String,
        image: String,
        moves: Vec<Vec<bool>>,
        kills: Vec<Vec<bool>>,
    ) -> Result<Piece> {
        Ok(Piece {
            piece_name: name,
            image_path: image,
            move_set: moves,
            kill_set: kills,
        })
    }

    pub fn get_name(&mut self) -> String {
        return self.piece_name;
    }

    pub fn set_kill_status(&mut self) {
        return self.image_path;
    }

    pub fn get_move_set(&mut self) -> Vec<Vec<bool>> {
        return self.move_set;
    }

    pub fn set_move_status(&mut self) -> Vec<Vec<bool>> {
        return self.kill_set;
    }
}
