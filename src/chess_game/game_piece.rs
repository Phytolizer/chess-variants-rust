pub struct GamePiece {
    pub piece_name: String, // Needs to be reference (lifetime?)
    pub team_name: String,
    pub horz_position: char,
    pub vert_position: u32,
}

impl GamePiece {
    pub fn new(
        piece: String,
        team: String,
        horz: char,
        vert: u32,
    ) -> Result<GamePiece, crate::Error> {
        Ok(GamePiece {
            piece_name: piece,
            team_name: team,
            horz_position: horz,
            vert_position: vert,
        })
    }
}
