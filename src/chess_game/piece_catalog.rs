use std::io::BufRead;
use std::io::BufReader;
use std::{collections::HashMap, fs};
use std::{fmt::Display, fs::File};

use super::{piece::Piece, InvalidFormatError};

pub struct PieceCatalog {
    catalog: HashMap<String, Piece>,
}

impl PieceCatalog {
    pub fn new() -> Result<PieceCatalog, crate::Error> {
        Ok(PieceCatalog {
            catalog: HashMap::new(),
        })
    }

    pub fn generate(&mut self, dir_path: String) -> Result<(), crate::Error> {
        let dir = fs::read_dir(dir_path)?;
        for path in dir {
            let file = path?;
            if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
                let reader = BufReader::new(File::open(file.path())?);
                let mut piece = Piece::new()?;
                let mut line_num = 1;
                for line in reader.lines() {
                    let line = line?;
                    if line.starts_with('-') {
                        continue;
                    } else if line.starts_with("Name") {
                        piece.name = line
                            .split_whitespace()
                            .nth(1)
                            .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                            .to_string();
                    } else if line.starts_with("Image") {
                        piece.image_key = line
                            .split_whitespace()
                            .nth(1)
                            .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                            .to_string();
                    } else if line.starts_with("Leap") {
                        let (forward, left) = parse_move(line)?;
                        piece.add_leap(forward, left);
                    } else if line.starts_with("Kill") {
                        let (forward, left) = parse_move(line)?;
                        piece.add_kill(forward, left);
                    } else if line.starts_with("Run") {
                        let (forward, left) = parse_move(line)?;
                        piece.add_run(forward, left);
                    } else if line.starts_with("Special") {
                        let special_move = line
                            .split_whitespace()
                            .nth(1)
                            .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                            .to_string();
                        piece.add_special(special_move)?;
                    } else if line.starts_with("Promotion") {
                        let promotion_piece = line
                            .split_whitespace()
                            .nth(1)
                            .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                            .to_string();
                        piece.promotions.push(promotion_piece);
                    }
                    line_num += 1;
                }
                self.catalog.insert(piece.name.clone(), piece);
            }
        }
        Ok(())
    }

    pub fn get_piece(&self, piece_name: String) -> Result<&Piece, crate::Error> {
        let out = self
            .catalog
            .get(&piece_name)
            .ok_or_else(|| PieceNotFoundError { name: piece_name })?;
        Ok(out)
    }
}

fn parse_move(line: String) -> Result<(i32, i32), crate::Error> {
    let mv = line.split_whitespace().skip(1).collect::<Vec<_>>();
    Ok((mv[0].parse()?, mv[1].parse()?))
}

#[derive(Debug)]
pub struct PieceNotFoundError {
    name: String,
}

impl Display for PieceNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Piece not found: {}", self.name)
    }
}

impl std::error::Error for PieceNotFoundError {}
