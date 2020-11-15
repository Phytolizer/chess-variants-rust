pub use board::Board;
pub use chess_settings::ChessSettings;
pub use piece_catalog::PieceCatalog;

pub struct ChessGame {
    pub piece_catalog: PieceCatalog,
    pub board: Board,
}

impl ChessGame {
    pub fn new(width: u32, height: u32) -> Result<ChessGame> {
        Ok(ChessGame {
            piece_catalog: PieceCatalog::new(),
            board: Board::new(),
        })
    }

    pub fn load_pieces_from_folder(path: String) {
        // For each file in folder
        let dir = fs::read_dir(path)?;
        for file in dir {
            // Check if the file is valid
            let file = file?;
            if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
                // Set piece information from the file
                let name: String = "";
                let image_path: String = "";
                let move_set: Vec<Vec<bool>> = Vec![];
                let kill_set: Vec<Vec<bool>> = "";
                for line in file_to_lines_iter(file.path())? {
                    if (line.starts_with("name")) {
                        name = line;
                        continue;
                    } else if (line.starts_with("image")) {
                        image_path = line;
                        continue;
                    } else if (line.starts_with("move")) {
                        move_set.append(line);
                        continue;
                    } else if (line.starts_with("kill")) {
                        kill_set.append(line);
                        continue;
                    }
                }
                self.piece_catalog
                    .add_piece(Piece::new(name, move_set, kill_set, image_path));
            }
        }
        pub fn load_board_from_file(path: String) {}
    }
}
