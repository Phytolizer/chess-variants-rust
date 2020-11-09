use super::piece::Piece;
use std::io::{BufRead, BufReader};

pub struct PieceCatalog {
    catalog: Vec<Piece>,
}

impl PieceCatalog {
    pub fn new() -> Result<PieceCatalog> {
        Ok(PieceCatalog { catalog: Vec![] })
    }

    pub fn generate(&mut self, dir_path: String) {
        let dir = fs::read_dir(dir_path)?;
        for file in dir {
            let file = file?;
            if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
                let reader = BufReader::new(file);
                let piece = Piece::new();
                for line in reader.lines() {
                    if line.starts_with("-") {
                        continue;
                    } else if line.starts_with("Name") {
                        // Name: Rook
                        piece.name = line;
                    } else if line.starts_with("Image") {
                        // Image: Bishop.png
                        piece.image = line;
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
    }
}
