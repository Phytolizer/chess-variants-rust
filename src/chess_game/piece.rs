pub struct Piece {
    pub name: String,
    pub image_path: String,
    pub move_set: Vec<Vec<u32>>,
    pub kill_set: Vec<Vec<u32>>,
}

// pub struct Move {}

impl Piece {
    pub fn new() -> Piece {
        Ok(Piece {
            name: "",
            image_path: "",
            move_set: vec![],
            kill_set: vec![],
        })
    }
}
