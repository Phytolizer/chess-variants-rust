mod gridsquare;
mod piece;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::render::TextureCreator;
use sdl2::render::TextureValueError;

use std::error::Error;

pub struct Chess<'tc, T> {
    pieces: Vec<piece::Piece>,
    pub grid: ChessGrid<'tc, T>,
}

pub struct ChessGrid<'tc, T> {
    grid: Vec<gridsquare::GridSquare>,
    pub texture: Texture<'tc>,
    texture_creator: &'tc TextureCreator<T>,
    pub size_horz: u32,
    pub size_vert: u32,
    pub off_horz: u32,
    pub off_vert: u32,
}

impl<'tc, T> ChessGrid<'tc, T> {
    pub fn new(
        texture_creator: &'tc TextureCreator<T>,
        width: u32,
        height: u32,
    ) -> Result<ChessGrid<'tc, T>, TextureValueError>
    where
        T: 'tc,
    {
        let texture = texture_creator.create_texture(None, TextureAccess::Target, width, height)?;
        Ok(ChessGrid {
            grid: vec![],
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
        size_h: u32,
        size_v: u32,
        canvas: &mut Canvas<RT>,
    ) -> Result<(), Box<dyn Error>>
    where
        RT: RenderTarget,
    {
        let grid = &self.grid;

        //self.size_horz = size_h;
        //self.size_vert = size_v;
        //self.off_horz = (width - size_h) / 2;
        //self.off_vert = (height - size_v) / 2;

        self.texture =
            self.texture_creator
                .create_texture(None, TextureAccess::Target, width, height)?;

        canvas.with_texture_canvas(&mut self.texture, |canvas: &mut Canvas<RT>| {
            canvas.set_draw_color(Color::RGB(0x77, 0x77, 0x77));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0x33, 0x33, 0x33));
            for (i, square) in grid.iter().enumerate() {
                if square.pos_horz % 2 == square.pos_vert % 2 {
                    canvas
                        .fill_rect(Rect::new(
                            (square.pos_horz as f32 * square.square_size) as i32,
                            (square.pos_vert as f32 * square.square_size) as i32,
                            square.square_size as u32,
                            square.square_size as u32,
                        ))
                        .unwrap();
                }
            }
        })?;

        Ok(())
    }
}

impl<'tc, T> Chess<'tc, T> {
    pub fn new(
        texture_creator: &'tc TextureCreator<T>,
        width: u32,
        height: u32,
    ) -> Result<Chess<'tc, T>, TextureValueError>
    where
        T: 'tc,
    {
        Ok(Chess {
            grid: ChessGrid::new(texture_creator, width, height)?,
            pieces: vec![],
        })
    }

    pub fn update_grid<RT>(
        &mut self,
        squares_horz: u32,
        squares_vert: u32,
        width: u32,
        height: u32,
        canvas: &mut Canvas<RT>,
    ) -> Result<(), Box<dyn Error>>
    where
        RT: RenderTarget,
    {
        println!("Method entry update_grid");
        let squares_size: u32;
        if width / squares_horz > height / squares_vert {
            squares_size = width / squares_horz;
        } else {
            squares_size = height / squares_vert;
        }
        let mut new_grid: Vec<gridsquare::GridSquare> = vec![];
        for vert in 0..squares_vert {
            for horz in 0..squares_horz {
                new_grid.push(gridsquare::GridSquare::new(vert, horz, squares_size));
                dbg!((vert, horz));
            }
        }
        self.grid.grid = new_grid;
        self.grid.redraw(
            width,
            height,
            squares_size * squares_horz,
            squares_size * squares_vert,
            canvas,
        )?;

        Ok(())
    }
}
