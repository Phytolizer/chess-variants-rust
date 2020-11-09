use std::sync::Arc;

use sdl2::render::{Canvas, RenderTarget, Texture};

use crate::sdl_error::ToSdl;

pub struct Piece<'tc> {
    pub team_number: u32,
    pub position_horz: u32,
    pub position_vert: u32,
    pub selected: bool,
    pub has_crown: bool,
    pub texture: Arc<Texture<'tc>>,
}

impl<'tc> Piece<'tc> {
    pub fn new(team: u32, pos_horz: u32, pos_vert: u32, texture: Arc<Texture<'tc>>) -> Piece<'tc> {
        Piece {
            team_number: team,
            position_horz: pos_horz,
            position_vert: pos_vert,
            selected: false,
            has_crown: false,
            texture,
        }
    }

    pub fn display<RT: RenderTarget>(&self, canvas: &mut Canvas<RT>) -> Result<(), crate::Error> {
        // TODO properly scale the piece to the board
        canvas
            .copy(
                &self.texture,
                None,
                sdl2::rect::Rect::new(
                    self.position_horz as i32,
                    self.position_vert as i32,
                    100,
                    100,
                ),
            )
            .sdl_error()?;
        Ok(())
    }
}
