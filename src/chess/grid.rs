use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::render::TextureCreator;
use sdl2::render::TextureValueError;

pub struct Grid<'tc, T> {
    pub texture: Texture<'tc>,
    texture_creator: &'tc TextureCreator<T>,
    pub size_horz: u32,
    pub size_vert: u32,
    pub off_horz: i32,
    pub off_vert: i32,
}

impl<'tc, T> Grid<'tc, T> {
    pub fn new(
        texture_creator: &'tc TextureCreator<T>,
        width: u32,
        height: u32,
    ) -> Result<Grid<'tc, T>, TextureValueError>
    where
        T: 'tc,
    {
        let texture = texture_creator.create_texture(None, TextureAccess::Target, width, height)?;
        Ok(Grid {
            texture,
            texture_creator,
            size_horz: 0,
            size_vert: 0,
            off_horz: 0,
            off_vert: 0,
        })
    }

    pub fn redraw<RT>(
        &mut self,
        width: u32,
        height: u32,
        settings: &mut super::ChessSettings,
        canvas: &mut Canvas<RT>,
    ) -> Result<(), Box<dyn Error>>
    where
        RT: RenderTarget,
    {
        let squares_size: u32;
        if width / settings.squares_horz < height / settings.squares_vert {
            squares_size = width / settings.squares_horz;
        } else {
            squares_size = height / settings.squares_vert;
        }

        settings.squares_size = squares_size;
        self.size_horz = settings.squares_horz * squares_size;
        self.size_vert = settings.squares_vert * squares_size;
        self.off_horz = ((width - self.size_horz) / 2) as i32;
        self.off_vert = ((height - self.size_vert) / 2) as i32;

        self.texture = self.texture_creator.create_texture(
            None,
            TextureAccess::Target,
            self.size_horz,
            self.size_vert,
        )?;

        canvas.with_texture_canvas(&mut self.texture, |canvas: &mut Canvas<RT>| {
            canvas.set_draw_color(Color::RGB(0x80, 0x80, 0x80));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0x40, 0x40, 0x40));
            for horz in 0..settings.squares_horz {
                for vert in 0..settings.squares_vert {
                    if horz % 2 == vert % 2 {
                        canvas
                            .fill_rect(Rect::new(
                                (horz * squares_size) as i32,
                                (vert * squares_size) as i32,
                                squares_size,
                                squares_size,
                            ))
                            .unwrap();
                    }
                }
            }
        })?;

        Ok(())
    }
}
