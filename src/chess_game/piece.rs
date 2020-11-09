use super::piece_move::PieceMove;

pub struct Piece {
    pub name: String,
    pub image_key: String,
    pub move_set: Vec<PieceMove>,
    pub kill_set: Vec<PieceMove>,
}

// pub struct Move {}

impl Piece {
    pub fn new() -> Result<Piece, crate::Error> {
        Ok(Piece {
            name: "".to_string(),
            image_key: "".to_string(),
            move_set: vec![],
            kill_set: vec![],
        })
    }

    pub fn add_leap(&mut self, forward: u32, left: u32) {
        self.move_set.push(PieceMove::new_leap(forward, left));
    }
    pub fn add_kill(&mut self, forward: u32, left: u32) {
        self.move_set.push(PieceMove::new_kill(forward, left));
    }
    pub fn add_run(&mut self, forward: u32, left: u32) {
        self.move_set.push(PieceMove::new_run(forward, left));
    }
}
