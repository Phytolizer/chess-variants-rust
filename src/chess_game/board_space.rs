use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl_helpers::SdlError;

use super::game_piece::GamePiece;

pub struct BoardSpace {
    pub horz_position: u32,
    pub vert_position: u32,
    pub game_pieces: Vec<GamePiece>,
    pub hovered: bool,
    pub is_active: bool,
    pub available_to_move: bool,
    pub available_to_kill: bool,
    pub is_danger: bool,
    pub color: Color,
    pub rect: Rect,
}

impl BoardSpace {
    pub fn new(horz: u32, vert: u32, color: Color, rect: Rect) -> Result<BoardSpace, crate::Error> {
        Ok(BoardSpace {
            horz_position: horz,
            vert_position: vert,
            game_pieces: vec![],
            hovered: false,
            is_active: true,
            available_to_move: false,
            available_to_kill: false,
            is_danger: false,
            color,
            rect,
        })
    }

    #[allow(dead_code)]
    pub fn reset_status(&mut self) {
        self.available_to_move = false;
        self.available_to_kill = false;
        self.is_danger = false;
    }

    pub fn update_rect(&mut self, offset_x: i32, offset_y: i32, square_size: u32) {
        self.rect = Rect::new(
            offset_x + (self.horz_position * square_size) as i32,
            offset_y + (self.vert_position * square_size) as i32,
            square_size,
            square_size,
        );
    }

    pub fn draw(&self, canvas: &mut Canvas<impl RenderTarget>) -> Result<(), crate::Error> {
        let color = if self.available_to_kill {
            Color::RGBA(255, 0, 0, 100)
        } else if self.available_to_move {
            Color::RGBA(0, 0, 255, 100)
        } else {
            return Ok(());
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(self.rect).map_err(SdlError::Drawing)?;
        Ok(())
    }
}
