#[derive(Debug)]
pub struct Piece {
    // pub position: Position,
}

impl Piece {
    pub fn new() -> Piece {
        Piece {}
    }

    pub fn showThing(&self) {
        dbg!(self);
    }

    pub fn changeThing(&mut self) {}
}

fn test() {
    let p = Piece::new();
    p.showThing();
}
