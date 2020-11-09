use std::fs::DirEntry;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use super::game_piece::GamePiece;
use super::piece_catalog::PieceCatalog;
use super::{board_space::BoardSpace, InvalidFormatError};

pub struct Board {
    pub name: String,
    pub grid: Vec<BoardSpace>,
    pub players: Vec<String>, // TODO: Enum?
    pub game_pieces: Vec<GamePiece>,
    pub dead_pieces: Vec<GamePiece>,
}

impl Board {
    pub fn new() -> Result<Board, crate::Error> {
        Ok(Board {
            name: "".to_string(),
            grid: vec![],
            players: vec![],
            game_pieces: vec![],
            dead_pieces: vec![],
        })
    }

    pub fn generate(
        &mut self,
        file: DirEntry,
        chess_pieces: &PieceCatalog,
    ) -> Result<(), crate::Error> {
        if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
            let reader = BufReader::new(File::open(file.path())?);
            let mut line_num = 1;
            for line in reader.lines() {
                let line = line?;
                if line.starts_with("-") {
                    continue;
                } else if line.starts_with("Name") {
                    // Name: Classic Chess
                    self.name = line;
                } else if line.starts_with("Size") {
                    // Size: 8 8
                    let horz_size = line;
                    let vert_size = line;
                // Generate blank board_space to self.grid
                } else if line.starts_with("Player") {
                    // Players: white
                    self.players.push(line);
                } else if line.starts_with("Disabled") {
                    // Update board_space at location in grid and disable
                } else if line.starts_with("Piece") {
                    let line = line.split_whitespace().take(1);
                    let horz_pos = line
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num))?
                        .chars()
                        .next()
                        .unwrap();
                    let vert_pos = line
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num))?
                        .parse()?;
                    let team_name = line
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num))?
                        .to_string();
                    let piece = chess_pieces.get_piece(
                        line.next()
                            .ok_or_else(|| InvalidFormatError::new(line_num))?
                            .to_string(),
                    )?;
                    self.game_pieces
                        .push(GamePiece::new(piece.name, team_name, horz_pos, vert_pos)?);
                }
                line_num += 1;
            }
        }
        Ok(())
    }

    pub fn update(&mut self) {}
}
