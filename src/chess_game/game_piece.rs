use super::piece::Piece;

pub struct GamePiece<'p> {
    pub piece: &'p Piece, // Needs to be reference (lifetime?)
    pub team_name: String,
    pub horz_position: char,
    pub vert_position: u32,
}

impl<'p> GamePiece<'p> {
    pub fn new(
        p: &'p Piece,
        team: String,
        horz: char,
        vert: u32,
    ) -> Result<GamePiece<'p>, crate::Error> {
        Ok(GamePiece {
            piece: p,
            team_name: team,
            horz_position: horz,
            vert_position: vert,
        })
    }
}
