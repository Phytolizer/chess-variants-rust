pub struct Piece {
    pub name: String,
    pub image_path: String,
    pub move_set: Vec<Vec<u32>>,
    pub kill_set: Vec<Vec<u32>>,
}

// pub struct Move {}

impl Piece {
    pub fn new() -> Result<Piece, crate::Error> {
        Ok(Piece {
            name: "".to_string(),
            image_path: "".to_string(),
            move_set: vec![],
            kill_set: vec![],
        })
    }
}
