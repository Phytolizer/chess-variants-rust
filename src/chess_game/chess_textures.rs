use std::collections::HashMap;

use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Texture, TextureCreator, WindowCanvas},
};

use super::board_space::BoardSpace;

pub struct TextureRegistry<'tc, C> {
    texture_creator: &'tc TextureCreator<C>,
    board_texture: Option<Texture<'tc>>,
    pieces: HashMap<String, Texture<'tc>>,
}

impl<'tc, C> TextureRegistry<'tc, C> {
    pub fn new(texture_creator: &'tc TextureCreator<C>) -> Self {
        Self {
            texture_creator,
            board_texture: None,
            pieces: HashMap::new(),
        }
    }

    pub fn render_board(
        &mut self,
        canvas: &mut WindowCanvas,
        spaces: &Vec<BoardSpace>,
        board_width: u32,
        board_height: u32,
    ) -> Result<(), crate::Error> {
        let board_texture = self.texture_creator.create_texture_target(
            canvas.default_pixel_format(),
            board_width,
            board_height,
        )?;
        canvas.with_texture_canvas(&mut board_texture, |c| {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            for space in spaces {
                if !space.is_active {
                    continue;
                }
                canvas.set_draw_color(space.color);
                canvas.draw_point(Point::new());
            }
        });
        Ok(())
    }
}
