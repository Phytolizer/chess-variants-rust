mod gridsquare;
mod piece;

#[derive(Debug)]
pub struct Chess {
    pieces: Vec<piece::Piece>,
    grid: Vec<gridsquare::GridSquare>,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            grid: vec![],
            pieces: vec![],
        }
    }

    pub fn update_grid(&self, squares_horz: u32, squares_vert: u32, width: u32, height: u32) {
        let squares_size: u32;
        if width / squares_horz > height / squares_vert {
            squares_size = width / squares_horz;
        } else {
            squares_size = height / squares_vert;
        }
        let mut new_grid: Vec<gridsquare::GridSquare> = vec![];
        for vert in 0..squares_vert {
            for horz in 0..squares_horz {
                new_grid.push(gridsquare::GridSquare::new(vert, horz, squares_size));
            }
        }
    }
}

pub fn test_chess_test() {
    let pieces: Vec<piece::Piece>;
    let grid: Vec<gridsquare::GridSquare>;
}
