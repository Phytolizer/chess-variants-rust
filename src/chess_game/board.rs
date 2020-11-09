use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::board_space::BoardSpace;
use super::game_piece::GamePiece;
use super::piece_catalog::PieceCatalog;

pub struct Board<'p> {
    pub name: String,
    pub grid: Vec<BoardSpace>,
    pub players: Vec<String>, // TODO: Enum?
    pub game_pieces: Vec<GamePiece<'p>>,
    pub dead_pieces: Vec<GamePiece<'p>>,
}

impl<'p> Board<'p> {
    pub fn new() -> Result<Board<'p>, crate::Error> {
        Ok(Board {
            name: "".to_string(),
            grid: vec![],
            players: vec![],
            game_pieces: vec![],
            dead_pieces: vec![],
        })
    }

    pub fn generate(&mut self, dir_path: String, chess_pieces: &PieceCatalog) -> Result<(), crate::Error> {
        let dir = fs::read_dir(dir_path)?;
        for path in dir {
            let file = path?;
            if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
                let reader = BufReader::new(File::open(file.path())?);
                for line in reader.lines() {
                    let line = line?;
                    if line.starts_with("-") {
                        continue;
                    } else if line.starts_with("Name") {
                        // Name: Classic Chess
                        self.name = line;
                    } else if line.starts_with("Size") {
                        // Size: 8 8
                        let horzSize = line;
                        let vertSize = line;
                    // Generate blank board_space to self.grid
                    } else if line.starts_with("Player") {
                        // Players: white
                        self.players.push(line);
                    } else if line.starts_with("Disabled") {
                        // Update board_space at location in grid and disable
                    } else if line.starts_with("Piece") {
                        let horzPos = line;
                        let vertPos = line;
                        let team_name = line;
                        let piece = chess_pieces.find_piece(line);
                        self.game_pieces
                            .push(GamePiece::new(piece, team_name, horzPos, vertPos)?);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn update(&mut self) {}
}
