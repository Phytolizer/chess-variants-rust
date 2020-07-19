#![allow(dead_code)]
#[derive(Debug)]
pub struct Piece {
    // pub position: Position,
}

impl Piece {
    pub fn new() -> Piece {
        Piece {}
    }

    pub fn show_thing(&self) {
        dbg!(self);
    }
}

fn test() {
    let p = Piece::new();
    p.show_thing();
}
