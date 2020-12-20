use parking_lot::RwLock;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl_helpers::SdlError;
use std::fs::DirEntry;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;

use crate::chess_game::board::move_data::MoveData;
use crate::chess_game::texture_registry::UninitializedTextureRegistryError;

use super::board_space::BoardSpace;
use super::game_piece::GamePiece;
use super::piece::Piece;
use super::piece_catalog::PieceCatalog;
use super::piece_move::MoveRules;
use super::InvalidFormatError;

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
    pub rect: Rect,
}

struct SelectedPiece<'a> {
    piece: &'a Piece,
    horz_pos: i32,
    vert_pos: i32,
    team_name: String,
}

impl Board {
    pub fn new() -> Result<Board, crate::Error> {
        Ok(Board {
            name: "".to_string(),
            grid: vec![],
            players: vec![],
            dead_pieces: vec![],
            moves: vec![],
            width: 0,
            height: 0,
            space_size: 0,
            rect: Rect::new(0, 0, 0, 0),
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
                            self.grid.push(BoardSpace::new(
                                i,
                                j,
                                color,
                                Rect::new(
                                    self.rect.x + (self.space_size * i) as i32,
                                    self.rect.y + (self.space_size * j) as i32,
                                    self.space_size,
                                    self.space_size,
                                ),
                            )?);
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

    pub fn collect_game_pieces(&self) -> Vec<&GamePiece> {
        let mut game_pieces = vec![];
        for space in &self.grid {
            for piece in &space.game_pieces {
                game_pieces.push(piece);
            }
        }
        game_pieces
    }

    pub fn calculate_values(&mut self, horz_size: u32, vert_size: u32) {
        self.space_size = if horz_size / self.width < vert_size / self.height {
            horz_size / self.width
        } else {
            vert_size / self.height
        };

        self.rect.x = ((horz_size - self.width * self.space_size) / 2) as i32;
        self.rect.y = ((vert_size - self.height * self.space_size) / 2) as i32;
    }

    pub fn mouse_hover(&mut self, x: &i32, y: &i32) -> Result<(), crate::Error> {
        for grid_space in self.grid.iter_mut() {
            let rect = Rect::new(
                self.rect.x + (grid_space.horz_position * self.space_size) as i32,
                self.rect.y + (grid_space.vert_position * self.space_size) as i32,
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
                        team_name: piece.team_name.clone(),
                    });
                    println!("{}", piece.piece_name);
                }
            }
        }
        self.generate_moves_for(selected_pieces);
        Ok(())
    }

    fn generate_moves_for(&mut self, pieces: Vec<SelectedPiece>) {
        for sp in pieces {
            for mv in sp.piece.move_set.iter() {
                let (offset_x, offset_y) =
                    (mv.forward() + sp.horz_pos - 1, mv.left() + sp.vert_pos - 1);
                if offset_x < self.width as i32
                    && offset_x >= 0
                    && offset_y < self.height as i32
                    && offset_y >= 0
                {
                    match mv.movement_type() {
                        MoveRules::Leap => {
                            // Check if another piece is present at location
                            if let Some(space) = self.grid.iter_mut().find(|space| {
                                space.horz_position == offset_x as u32
                                    && space.vert_position == offset_y as u32
                            }) {
                                if space.game_pieces.is_empty() {
                                    space.available_to_move = true;
                                }
                            }
                        }
                        MoveRules::Run => {
                            // iterate over spaces until a piece is found or the square is invalid
                        }
                        MoveRules::Kill => {
                            if let Some(occupied_space) = self.grid.iter_mut().find(|p| {
                                p.horz_position == offset_x as u32
                                    && p.vert_position == offset_y as u32
                            }) {
                                // Check to see if the piece at this location is an ememy piece
                                if occupied_space
                                    .game_pieces
                                    .iter()
                                    .any(|p| p.team_name != sp.team_name)
                                {
                                    occupied_space.available_to_kill = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn render<'tc, R>(
        &mut self,
        canvas: Rc<RwLock<WindowCanvas>>,
        canvas_size: (u32, u32),
        texture_creator: &'tc TextureCreator<R>,
    ) -> Result<Texture<'tc>, crate::Error> {
        let mut board_texture = texture_creator.create_texture_target(
            canvas.read().default_pixel_format(),
            self.width,
            self.height,
        )?;

        self.calculate_values(canvas_size.0, canvas_size.1);

        let size_horz = self.width * self.space_size;
        let size_vert = self.height * self.space_size;

        self.rect = Rect::new(self.rect.x, self.rect.y, size_horz, size_vert);
        for square in self.grid.iter_mut() {
            square.update_rect(self.rect.x, self.rect.y, self.space_size);
        }
        canvas
            .write()
            .with_texture_canvas(&mut board_texture, |c: &mut WindowCanvas| {
                c.set_draw_color(Color::BLACK);
                c.clear();
                for space in &self.grid {
                    if !space.is_active {
                        continue;
                    }
                    c.set_draw_color(space.color);
                    c.draw_point(Point::new(
                        space.horz_position as i32,
                        space.vert_position as i32,
                    ))
                    // FIXME FIXME FIXME
                    .unwrap();
                }
            })?;
        Ok(board_texture)
    }
    pub fn draw(
        &self,
        canvas: Rc<RwLock<WindowCanvas>>,
        texture: &Option<Texture>,
    ) -> Result<(), crate::Error> {
        canvas
            .write()
            .copy(
                texture
                    .as_ref()
                    .ok_or(UninitializedTextureRegistryError {})?,
                None,
                Some(self.rect),
            )
            .map_err(SdlError::Drawing)?;
        for space in self.grid.iter() {
            space.draw(canvas.clone())?;
        }
        Ok(())
    }
}
