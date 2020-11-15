use sdl2::pixels::Color;
use std::{fs::DirEntry, fs::File, io::BufRead, io::BufReader};

use super::board_space::BoardSpace;
use super::game_piece::GamePiece;
use super::piece_catalog::PieceCatalog;
use super::InvalidFormatError;

pub struct Board {
    pub name: String,
    pub grid: Vec<BoardSpace>,
    pub width: u32,
    pub height: u32,
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
            width: 0,
            height: 0,
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
                if line.starts_with('-') {
                    continue;
                } else if line.starts_with("Name") {
                    // Name: Classic Chess
                    self.name = line;
                } else if line.starts_with("Size") {
                    // Size: 8 8
                    let mut line_iter = line.split_whitespace().skip(1);
                    self.width = line_iter
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                        .parse()?;
                    self.height = line_iter
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                        .parse()?;
                    for i in 0..self.width {
                        for j in 0..self.height {
                            let color = if i % 2 == j % 2 {
                                Color::BLACK
                            } else {
                                Color::WHITE
                            };
                            self.grid.push(BoardSpace::new(i, j, color)?);
                        }
                    }
                // Generate blank board_space to self.grid
                } else if line.starts_with("Player") {
                    // Players: white
                    self.players.push(line);
                } else if line.starts_with("Disabled") {
                    // Update board_space at location in grid and disable
                } else if line.starts_with("Piece") {
                    let mut line_iter = line.split_whitespace().skip(1);
                    let horz_pos = line_iter
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                        .parse()?;
                    let vert_pos = line_iter
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                        .parse()?;
                    let team_name = line_iter
                        .next()
                        .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                        .to_string();
                    let piece = chess_pieces.get_piece(
                        &line_iter
                            .next()
                            .ok_or_else(|| InvalidFormatError::new(line_num, line.clone()))?
                            .to_string(),
                    )?;
                    self.game_pieces.push(GamePiece::new(
                        piece.name.clone(),
                        team_name,
                        horz_pos,
                        vert_pos,
                    )?);
                }
                line_num += 1;
            }
        }
        Ok(())
    }
}
