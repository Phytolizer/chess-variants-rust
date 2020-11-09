use super::piece::Piece;

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
                let piece = Piece::new();
                for line in file {
                    if line.startsWith("-") {
                        continue;
                    } else if line.startsWith("Name") {
                        // Name: Rook
                        piece.name = line;
                    } else if line.startsWith("Image") {
                        // Image: Bishop.png
                        piece.image = line;
                    } else if line.startsWith("Move") {
                        // Move: 0 1
                        piece.move_set.push(line);
                    } else if line.startsWith("Kill") {
                        // Kill: 1 1
                        piece.kill_set.push(line);
                    }
                }
                self.catalog.push(piece);
            }
        }
    }
}
