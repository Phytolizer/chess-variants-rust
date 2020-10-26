use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

use crate::Error;

pub struct Widget {
    pub rect: Rect,
    pub color: Color,
}

pub trait Widgety {
    fn draw<RT>(&self, canvas: &mut Canvas<RT>) -> Result<(), Error>
    where
        RT: RenderTarget;
    fn handle_event(&mut self, event: Event) -> Result<(), Error>;
}

impl Widget {
    #[allow(dead_code)]
    pub fn new(rect: Option<Rect>) -> Widget {
        Widget {
            rect: if let Some(rect) = rect {
                rect
            } else {
                Rect::new(0, 0, 0, 0)
            },
            color: Color::BLACK,
        }
    }
}

impl Clone for Widget {
    fn clone(&self) -> Self {
        Self {
            rect: self.rect,
            color: self.color,
        }
    }
}
