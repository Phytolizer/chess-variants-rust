use sdl2::rect::Rect;
use std::{collections::HashMap, fmt::Display};

use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Texture, TextureCreator, WindowCanvas},
};

use crate::sdl_error::ToSdl;

use super::board::Board;
use super::piece_catalog::PieceCatalog;

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
        canvas: &mut WindowCanvas,
        canvas_size: (u32, u32),
        board: &Board,
    ) -> Result<(), crate::Error> {
        let mut board_texture = self.texture_creator.create_texture_target(
            canvas.default_pixel_format(),
            board.width,
            board.height,
        )?;

        let squares_size = if canvas_size.0 / board.width < canvas_size.1 / board.height {
            canvas_size.0 / board.width
        } else {
            canvas_size.1 / board.height
        };

        let off_horz = ((canvas_size.0 - board.width * squares_size) / 2) as i32;
        let off_vert = ((canvas_size.1 - board.height * squares_size) / 2) as i32;
        let size_horz = board.width * squares_size;
        let size_vert = board.height * squares_size;

        dbg!((off_horz, off_vert, canvas_size));

        self.area = Rect::new(off_horz, off_vert, size_horz, size_vert); // FIXME add offset
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

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        board: &Board,
        pieces: &PieceCatalog,
    ) -> Result<(), crate::Error> {
        canvas
            .copy(
                self.board_texture
                    .as_ref()
                    .ok_or_else(|| UninitializedTextureRegistryError {})?,
                None,
                Some(self.area),
            )
            .sdl_error()?;
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
