use sdl2::rect::Rect;
use std::{collections::HashMap, fmt::Display, fs, path::PathBuf};

use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::Point,
    render::{Texture, TextureCreator, WindowCanvas},
};

use crate::sdl_error::ToSdl;

use super::board::Board;

pub struct TextureRegistry<'tc, C> {
    pub texture_creator: &'tc TextureCreator<C>,
    pub board_texture: Option<Texture<'tc>>,
    pub pieces: HashMap<String, Texture<'tc>>,
    pub area: Rect,
    pub squares_size: u32,
    pub off_horz: i32,
    pub off_vert: i32,
}

impl<'tc, C> TextureRegistry<'tc, C> {
    pub fn new(texture_creator: &'tc TextureCreator<C>) -> Self {
        Self {
            texture_creator,
            board_texture: None,
            pieces: HashMap::new(),
            area: Rect::new(0, 0, 0, 0),
            squares_size: 0,
            off_horz: 0,
            off_vert: 0,
        }
    }

    pub fn render_board(
        &mut self,
        canvas: &mut WindowCanvas,
        canvas_size: (u32, u32),
        board: &Board,
    ) -> Result<(), crate::Error> {
        let mut board_texture = self.texture_creator.create_texture_target(
            canvas.default_pixel_format(),
            board.width,
            board.height,
        )?;

        self.squares_size = if canvas_size.0 / board.width < canvas_size.1 / board.height {
            canvas_size.0 / board.width
        } else {
            canvas_size.1 / board.height
        };

        self.off_horz = ((canvas_size.0 - board.width * self.squares_size) / 2) as i32;
        self.off_vert = ((canvas_size.1 - board.height * self.squares_size) / 2) as i32;
        let size_horz = board.width * self.squares_size;
        let size_vert = board.height * self.squares_size;

        self.area = Rect::new(self.off_horz, self.off_vert, size_horz, size_vert); // FIXME add offset
        canvas.with_texture_canvas(&mut board_texture, |c: &mut WindowCanvas| {
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

    pub fn render(&self, canvas: &mut WindowCanvas, board: &Board) -> Result<(), crate::Error> {
        canvas
            .copy(
                self.board_texture
                    .as_ref()
                    .ok_or_else(|| UninitializedTextureRegistryError {})?,
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
                self.off_horz + ((game_piece.horz_position - 1) * self.squares_size) as i32,
                self.off_vert + ((game_piece.vert_position - 1) * self.squares_size) as i32,
                self.squares_size,
                self.squares_size,
            );
            canvas
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
