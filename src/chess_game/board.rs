use sdl2::{pixels::Color, rect::Rect};
use std::{fs::DirEntry, fs::File, io::BufRead, io::BufReader};

use super::game_piece::GamePiece;
use super::piece_catalog::PieceCatalog;
use super::InvalidFormatError;
use super::{board_space::BoardSpace, piece::Piece};

pub(crate) mod move_data;

pub struct Board {
    pub name: String,
    pub grid: Vec<BoardSpace>,
    pub width: u32,
    pub height: u32,
    pub players: Vec<String>,
    pub dead_pieces: Vec<GamePiece>, // Maybe save the collection of MOVES and just remove dead pieces
    pub moves: Vec<MoveData>,
    pub space_size: u32,
    pub horz_offset: i32,
    pub vert_offset: i32,
}

struct SelectedPiece<'a> {
    piece: &'a Piece,
    horz_pos: i32,
    vert_pos: i32,
}

impl Board {
    pub fn new() -> Result<Board, crate::Error> {
        Ok(Board {
            name: "".to_string(),
            grid: vec![],
            players: vec![],
            dead_pieces: vec![],
            width: 0,
            height: 0,
            space_size: 0,
            horz_offset: 0,
            vert_offset: 0,
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
                    let piece_index = self.find_board_space_index(horz_pos, vert_pos);
                    if piece_index >= 0 {
                        let game_pieces =
                            &mut self.grid.get_mut(piece_index as usize).unwrap().game_pieces;
                        game_pieces.push(GamePiece::new(
                            piece.name.clone(),
                            team_name,
                            horz_pos,
                            vert_pos,
                        )?);
                    }
                }
                line_num += 1;
            }
        }
        Ok(())
    }

    pub fn find_board_space_index(&self, horz_pos: u32, vert_pos: u32) -> i32 {
        self.grid
            .iter()
            .position(|sp| sp.horz_position == horz_pos - 1 && sp.vert_position == vert_pos - 1)
            .map(|x| x as i32)
            .unwrap_or(-1)
    }

    pub fn collect_game_pieces(&self) -> Result<Vec<&GamePiece>, crate::Error> {
        let mut game_pieces = vec![];
        for space in &self.grid {
            for piece in &space.game_pieces {
                game_pieces.push(piece);
            }
        }
        Ok(game_pieces)
    }

    pub fn calculate_values(&mut self, horz_size: u32, vert_size: u32) -> Result<(), crate::Error> {
        self.space_size = if horz_size / self.width < vert_size / self.height {
            horz_size / self.width
        } else {
            vert_size / self.height
        };

        self.horz_offset = ((horz_size - self.width * self.space_size) / 2) as i32;
        self.vert_offset = ((vert_size - self.height * self.space_size) / 2) as i32;
        Ok(())
    }

    pub fn mouse_hover(&mut self, x: &i32, y: &i32) -> Result<(), crate::Error> {
        for grid_space in self.grid.iter_mut() {
            let rect = Rect::new(
                self.horz_offset + (grid_space.horz_position * self.space_size) as i32,
                self.vert_offset + (grid_space.vert_position * self.space_size) as i32,
                self.space_size,
                self.space_size,
            );
            if rect.contains_point((*x, *y)) {
                grid_space.hovered = true;
            } else {
                grid_space.hovered = false;
            }
        }
        Ok(())
    }

    pub fn mouse_left_click(&mut self, pieces: &PieceCatalog) -> Result<(), crate::Error> {
        for sp in &mut self.grid {
            for piece in &mut sp.game_pieces {
                piece.selected = sp.hovered;
            }
        }
        let mut selected_pieces = vec![];
        for sp in &self.grid {
            for piece in &sp.game_pieces {
                if piece.selected {
                    let p = pieces.get_piece(&piece.piece_name)?;
                    selected_pieces.push(SelectedPiece {
                        piece: p,
                        horz_pos: piece.horz_position as i32,
                        vert_pos: piece.vert_position as i32,
                    });
                    println!("{}", piece.piece_name);
                }
            }
        }
        self.generate_moves_for(selected_pieces)?;
        Ok(())
    }

    fn generate_moves_for(&mut self, pieces: Vec<SelectedPiece>) -> Result<(), crate::Error> {
        for sp in pieces {
            for mv in sp.piece.move_set.iter() {
                let (offset_x, offset_y) = (mv.forward() + sp.horz_pos, mv.left() + sp.vert_pos);
                if offset_x < self.width as i32
                    && offset_x >= 0
                    && offset_y < self.height as i32
                    && offset_y >= 0
                {
                    match mv.movement_type() {
                        super::piece_move::MoveRules::Leap => {
                            // Check if another piece is present at location
                            if self
                                .grid
                                .iter()
                                .find(|p| {
                                    p.horz_position == offset_x as u32
                                        && p.vert_position == offset_y as u32
                                })
                                .is_some()
                            {
                                // there is a piece at this location
                            } else {
                                // empty location
                            }
                        }
                        super::piece_move::MoveRules::Run => {
                            // iterate over spaces until a piece is found or the square is invalid
                        }
                        super::piece_move::MoveRules::Kill => {
                            if let Some(occupied_space) = self.grid.iter().find(|p| {
                                p.horz_position == offset_x as u32
                                    && p.vert_position == offset_y as u32
                            }) {
                                // Check to see if the piece at this location is an ememy piece
                                if occupied_space
                                    .game_pieces
                                    .iter()
                                    .any(|p| p.team_name != todo!("what goes here?"))
                                {
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}
