use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::piece::Piece;

pub struct PieceCatalog {
    catalog: Vec<Piece>,
}

impl PieceCatalog {
    pub fn new() -> Result<PieceCatalog, crate::Error> {
        Ok(PieceCatalog { catalog: vec![] })
    }

    pub fn generate(&mut self, dir_path: String) -> Result<(), crate::Error> {
        let dir = fs::read_dir(dir_path)?;
        for path in dir {
            let file = path?;
            if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
                let reader = BufReader::new(File::open(file.path())?);
                let piece = Piece::new()?;
                for line in reader.lines() {
                    let line = line?;
                    if line.starts_with("-") {
                        continue;
                    } else if line.starts_with("Name") {
                        // Name: Rook
                        piece.name = line;
                    } else if line.starts_with("Image") {
                        // Image: Bishop.png
                        piece.image_path = line;
                    } else if line.starts_with("Move") {
                        // Move: 0 1
                        piece.move_set.push(line);
                    } else if line.starts_with("Kill") {
                        // Kill: 1 1
                        piece.kill_set.push(line);
                    }
                }
                self.catalog.push(piece);
            }
        }
        Ok(())
    }
}
