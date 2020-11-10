use super::piece_move::PieceMove;
use super::{piece_move::MoveRules, InvalidFormatError};

#[derive(Debug)]
pub struct Piece {
    pub name: String,
    pub image_key: String,
    pub move_set: Vec<PieceMove>,
    pub promotions: Vec<String>,
}

// pub struct Move {}

impl Piece {
    pub fn new() -> Result<Piece, crate::Error> {
        Ok(Piece {
            name: "".to_string(),
            image_key: "".to_string(),
            move_set: vec![],
            promotions: vec![],
        })
    }

    pub fn add_leap(&mut self, forward: i32, left: i32) {
        self.move_set
            .push(PieceMove::new(forward, left, MoveRules::Leap));
    }
    pub fn add_kill(&mut self, forward: i32, left: i32) {
        self.move_set
            .push(PieceMove::new(forward, left, MoveRules::Kill));
    }
    pub fn add_run(&mut self, forward: i32, left: i32) {
        self.move_set
            .push(PieceMove::new(forward, left, MoveRules::Run));
    }

    pub fn add_special(&mut self, special: String) -> Result<(), InvalidFormatError> {
        match special.parse::<MoveRules>() {
            Ok(MoveRules::PawnFirst) => self
                .move_set
                .push(PieceMove::new_special(MoveRules::PawnFirst)),
            Ok(MoveRules::Castle) => self
                .move_set
                .push(PieceMove::new_special(MoveRules::Castle)),
            Ok(_) => return Err(InvalidFormatError::new(0, special)),
            Err(_) => return Err(InvalidFormatError::new(0, special)),
        }
        Ok(())
    }
}
