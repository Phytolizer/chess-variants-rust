use board_space::BoardSpace;

pub struct Board {
    pub grid: Vec<BoardSpace>,
    pub size_horz: u32,
    pub size_vert: u32,
}

impl Board {
    pub fn new() -> Result<Board> {
        Ok(Board {
            grid: Vec![],
            size_horz: 0,
            size_vert: 0,
        })
    }

    pub fn adjust_size(&mut self, width: u32, height: u32) {
        
    }
}
