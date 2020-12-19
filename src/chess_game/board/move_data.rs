pub struct MoveData {
    player: usize,
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
}

impl MoveData {
    pub(crate) fn new(player: usize, from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Self {
        Self {
            player,
            from_x,
            from_y,
            to_x,
            to_y,
        }
    }

    pub(crate) fn player(&self) -> usize {
        self.player
    }

    pub(crate) fn from_x(&self) -> i32 {
        self.from_x
    }

    pub(crate) fn from_y(&self) -> i32 {
        self.from_y
    }

    pub(crate) fn to_x(&self) -> i32 {
        self.to_x
    }

    pub(crate) fn to_y(&self) -> i32 {
        self.to_y
    }
}
