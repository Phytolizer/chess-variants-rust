use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

use std::error::Error;

pub struct Widget {
    pub rect: Rect,
    pub color: Color,
}

pub trait Widgety {
    fn draw<RT>(&self, canvas: &mut Canvas<RT>) -> Result<(), Box<dyn Error>>
    where
        RT: RenderTarget;
    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn Error>>;
}

impl Widget {
    /*pub fn new(rect: Rect) -> Widget {
        Widget {
            rect,
            color: Color::BLACK,
        }
    }*/
}
