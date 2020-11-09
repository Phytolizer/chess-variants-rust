use super::piece::Piece;

pub struct GamePiece {
    piece: &Piece,
    team_name: String,
    horz_position: char,
    vert_position: u32,
}

impl GamePiece {
    pub fn new(p: &GamePiece, team: String, horz: char, vert: u32) -> Result<GamePiece> {
        Ok(GamePiece {
            piece: n,
            team_name: team,
            horz_position: horz,
            vert_position: vert,
        })
    }
}
