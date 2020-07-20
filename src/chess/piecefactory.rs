use super::piece::Piece;

use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Factory {
    pub factory: Vec<PieceFactory>,
}

#[derive(Debug)]
pub struct PieceFactory {
    pub piece_name: String,
    pub piece_movement: Vec<Vec<u32>>,
}

impl Factory {
    pub fn new() -> Factory {
        Factory { factory: vec![] }
    }

    pub fn build_piece(
        &mut self,
        team: u32,
        piece_type: &str,
        pos_horz: u32,
        pos_vert: u32,
    ) -> Piece {
        match piece_type {
            _ => {
                let piece_factory = self.factory.choose_mut(&mut rand::thread_rng()).unwrap();
                let piece = piece_factory.build(team, pos_horz, pos_vert);
                return piece;
            }
        }
    }
}

impl PieceFactory {
    pub fn new(name: String, movement: Vec<Vec<u32>>) -> PieceFactory {
        PieceFactory {
            piece_name: name,
            piece_movement: movement,
        }
    }

    pub fn build(&mut self, team: u32, pos_horz: u32, pos_vert: u32) -> Piece {
        let new_piece: Piece = Piece::new(team, pos_horz, pos_vert);
        return new_piece;
    }
}
