use super::piece::Piece;

pub struct GamePiece {
    piece: &Piece,
    horz_position: char,
    vert_position: u32,
    team_name: String,
}

impl GamePiece {
    pub fn new(p: &GamePiece, horz: char, vert: u32, team: String) -> Result<GamePiece> {
        Ok(GamePiece {
            piece: n,
            horz_position: horz,
            vert_position: vert,
            team_name: team,
        })
    }

    pub fn get_horz(&mut self) -> char {
        return self.horz_position;
    }

    pub fn get_vert(&mut self) -> u32 {
        return self.vert_position;
    }

    pub fn get_team(&mut self) -> String {
        return self.team_name;
    }

    pub fn set_horz(&mut self, horz: char) {
        self.horz_position = horz;
    }

    pub fn set_vert(&mut self, vert: u32) {
        self.vert_position = vert;
    }
}
