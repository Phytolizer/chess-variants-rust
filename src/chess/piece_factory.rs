use super::piece::Piece;

#[derive(Debug)]
pub struct PieceFactory {
    pub piece_name: String,
    pub piece_movement: Vec<Vec<i32>>,
    pub piece_image: Vec<Vec<bool>>,
}

impl PieceFactory {
    pub fn new(name: String, movement: Vec<Vec<i32>>, image: Vec<Vec<bool>>) -> PieceFactory {
        PieceFactory {
            piece_name: name,
            piece_movement: movement,
            piece_image: image,
        }
    }

    pub fn build_piece(&mut self, team: u32, pos_horz: u32, pos_vert: u32) -> Piece {
        let new_piece: Piece = Piece::new(team, pos_horz, pos_vert);
        return new_piece;
    }
}

use std::io::BufRead;
fn file_to_lines<P: AsRef<std::path::Path>>(file_name: P) -> std::io::Result<Vec<String>> {
    let f = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(f);
    let mut err: std::io::Result<()> = Ok(());
    let contents: Vec<String> = reader
        .lines()
        .map(|l| match l {
            Ok(l) => l.to_owned(),
            Err(e) => {
                err = Err(e);
                String::new()
            }
        })
        .collect();
    err?;
    Ok(contents)
}

pub fn new_piece_factory(
    file: std::fs::DirEntry,
) -> Result<PieceFactory, Box<dyn std::error::Error>> {
    let mut piece_factory: PieceFactory = PieceFactory::new("name".to_string(), vec![], vec![]);
    let mut mode: String = "".to_string();
    for line in file_to_lines(file.path())? {
        if line == "" {
            continue;
        }
        if line == "start_name" {
            mode = "name".to_string();
        }
        if line == "start_moves" {
            mode = "move".to_string();
        }
        if line == "start_image" {
            mode = "image".to_string();
        }
        if mode == "name" {
            piece_factory.piece_name = line.clone();
        }
        if mode == "move" {
            //let parts = line.split_whitespace().map(|l| l.parse::<i32>());
            //let movement: Result<Vec<i32>, _> = parts.collect();
            //let movement = movement?;
            //piece_factory.piece_movement.push(movement);
        }
        if mode == "image" {}
    }
    piece_factory.piece_movement.push(vec![0, 1, 1]);
    piece_factory.piece_image.push(vec![true, false]);
    piece_factory.piece_image.push(vec![false, true]);
    return Ok(piece_factory);
}
