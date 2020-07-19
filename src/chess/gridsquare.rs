#[derive(Debug)]
pub struct GridSquare {
    pos_horz: u32,
    pos_vert: u32,
    square_size: f32,
}

impl GridSquare {
    pub fn new(horz: u32, vert: u32, size: u32) -> GridSquare {
        GridSquare {
            pos_horz: horz,
            pos_vert: vert,
            square_size: size as f32,
        }
    }

    pub fn showThing(&self) {
        dbg!(self);
    }

    pub fn changeThing(&mut self) {}
}
