#[derive(Debug)]
pub struct GridSquare {
    pub pos_horz: u32,
    pub pos_vert: u32,
    pub square_size: f32,
}

impl GridSquare {
    pub fn new(horz: u32, vert: u32, size: u32) -> GridSquare {
        GridSquare {
            pos_horz: horz,
            pos_vert: vert,
            square_size: size as f32,
        }
    }
}
