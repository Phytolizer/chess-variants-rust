use super::piece::Piece;

pub struct GamePiece {
    piece: Piece, // Needs to be reference (lifetime?)
    team_name: String,
    horz_position: char,
    vert_position: u32,
}

impl GamePiece {
    pub fn new(
        p: &GamePiece,
        team: String,
        horz: char,
        vert: u32,
    ) -> Result<GamePiece, crate::Error> {
        Ok(GamePiece {
            piece: p,
            team_name: team,
            horz_position: horz,
            vert_position: vert,
        })
    }
}
