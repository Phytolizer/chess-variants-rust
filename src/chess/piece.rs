#![allow(dead_code)]
#[derive(Debug)]
pub struct Piece {
    pub team_number: u32,
    pub position_horz: u32,
    pub position_vert: u32,
    pub selected: bool,
    pub has_crown: bool,
}

impl Piece {
    pub fn new(team: u32, pos_horz: u32, pos_vert: u32) -> Piece {
        Piece {
            team_number: team,
            position_horz: pos_horz,
            position_vert: pos_vert,
            selected: false,
            has_crown: false,
        }
    }

    pub fn show_thing(&self) {
        dbg!(self);
    }
}
