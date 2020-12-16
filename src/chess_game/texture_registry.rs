use crate::sdl_error::ToSdl;
use parking_lot::RwLock;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, WindowCanvas},
};
use std::{collections::HashMap, fmt::Display, fs, path::PathBuf, rc::Rc};

use super::board::Board;

pub struct TextureRegistry<'tc, C> {
    pub texture_creator: &'tc TextureCreator<C>,
    pub board_texture: Option<Texture<'tc>>,
    pub pieces: HashMap<String, Texture<'tc>>,
    pub area: Rect,
}

impl<'tc, C> TextureRegistry<'tc, C> {
    pub fn new(texture_creator: &'tc TextureCreator<C>) -> Self {
        Self {
            texture_creator,
            board_texture: None,
            pieces: HashMap::new(),
            area: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn render_board(
        &mut self,
        canvas: Rc<RwLock<WindowCanvas>>,
        canvas_size: (u32, u32),
        board: &mut Board,
    ) -> Result<(), crate::Error> {
        let mut board_texture = self.texture_creator.create_texture_target(
            canvas.read().default_pixel_format(),
            board.width,
            board.height,
        )?;

        board.calculate_values(canvas_size.0, canvas_size.1)?;

        let size_horz = board.width * board.space_size;
        let size_vert = board.height * board.space_size;

        self.area = Rect::new(board.horz_offset, board.vert_offset, size_horz, size_vert); // FIXME add offset
        canvas
            .write()
            .with_texture_canvas(&mut board_texture, |c: &mut WindowCanvas| {
                c.set_draw_color(Color::BLACK);
                c.clear();
                for space in &board.grid {
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
        self.board_texture = Some(board_texture);
        Ok(())
    }

    pub fn generate_piece_images(&mut self, dir_path: String) -> Result<(), crate::Error> {
        let dir = fs::read_dir(&dir_path)?;
        for path in dir {
            let file = path?;
            if file.file_type()?.is_file()
                && regex::Regex::new("\\.(png|jpg|jpeg)$")
                    .unwrap()
                    .is_match(&file.file_name().to_string_lossy())
            {
                let full_file_name = file.file_name().to_string_lossy().to_string();
                let tex = self
                    .texture_creator
                    .load_texture(&PathBuf::from(&dir_path).join(&full_file_name))
                    .sdl_error()?;
                let key = full_file_name.split('.').next().unwrap();
                self.pieces.insert(key.to_string(), tex);
            }
        }
        Ok(())
    }

    pub fn render(
        &self,
        canvas: Rc<RwLock<WindowCanvas>>,
        board: &Board,
    ) -> Result<(), crate::Error> {
        canvas
            .write()
            .copy(
                self.board_texture
                    .as_ref()
                    .ok_or(UninitializedTextureRegistryError {})?,
                None,
                Some(self.area),
            )
            .sdl_error()?;
        for game_piece in &board.game_pieces {
            let piece_texture = match self.pieces.get(&game_piece.piece_name) {
                Some(pt) => pt,
                None => continue,
            };
            let piece_area = Rect::new(
                board.horz_offset + ((game_piece.horz_position - 1) * board.space_size) as i32,
                board.vert_offset + ((game_piece.vert_position - 1) * board.space_size) as i32,
                board.space_size,
                board.space_size,
            );
            canvas
                .write()
                .copy(piece_texture, None, Some(piece_area))
                .sdl_error()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UninitializedTextureRegistryError {}

impl Display for UninitializedTextureRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uninitialized texture registry")
    }
}

impl std::error::Error for UninitializedTextureRegistryError {}
