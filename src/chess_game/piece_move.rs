pub(crate) enum PieceMove {
    Leap { forward: u32, left: u32 },
    Kill { forward: u32, left: u32 },
    Run { forward: u32, left: u32 },
}

impl PieceMove {
    pub fn new_leap(forward: u32, left: u32) -> Self {
        Self::Leap { forward, left }
    }
    pub fn new_kill(forward: u32, left: u32) -> Self {
        Self::Kill { forward, left }
    }
    pub fn new_run(forward: u32, left: u32) -> Self {
        Self::Run { forward, left }
    }
}
