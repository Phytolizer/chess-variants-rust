pub struct BoardSpace {
    pub horz_position: char,
    pub vert_position: i32,
    pub is_active: bool,
    pub available_to_move: bool,
    pub available_to_kill: bool,
    pub is_danger: bool,
}

impl BoardSpace {
    pub fn new(horz: char, vert: u32, active: bool) -> Result<BoardSpace> {
        Ok(BoardSpace {
            horz_position: horz,
            vert_position: vert,
            is_active: active,
            available_to_move: false,
            available_to_kill: false,
            is_danger: false,
        })
    }

    pub fn get_status(&mut self) -> bool {
        if is_active {
            return false;
        }
        return true;
    }

    pub fn reset_status(&mut self) {
        self.available_to_move = false;
        self.available_to_kill = false;
        self.is_danger = false;
    }

    pub fn set_move_status(&mut self, status: bool) {
        self.available_to_move = status;
    }

    pub fn set_kill_status(&mut self, status: bool) {
        available_to_kill = status;
    }

    pub fn set_danger_status(&mut self, status: bool) {
        is_danger = status;
    }
}
