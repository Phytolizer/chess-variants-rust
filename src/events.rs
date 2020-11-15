use std::rc::Rc;

use parking_lot::RwLock;
use sdl2::{event::Event, render::WindowCanvas};

use crate::{chess_game::ChessGame, gfx::Widgety};

pub struct EventHandler<'main, 'tc, C> {
    chess_game: Rc<RwLock<ChessGame<'tc, C>>>,
    canvas: Rc<RwLock<WindowCanvas>>,
    width: u32,
    height: u32,
}

impl<'main, 'tc, C> EventHandler<'main, 'tc, C> {
    pub fn new(
        chess_game: Rc<RwLock<ChessGame<'tc, C>>>,
        canvas: Rc<RwLock<WindowCanvas>>,
        widgets: &[&'main dyn Widgety],
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            chess_game,
            canvas,
            width,
            height,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), crate::Error> {
        match event {
            Event::RenderTargetsReset { .. } => {
                self.chess_game.write().textures.render_board(
                    self.canvas.clone(),
                    (self.width as u32, self.height as u32),
                    &self.chess_game.read().board,
                )?;
            }
            Event::Window { win_event, .. } => match win_event {
                sdl2::event::WindowEvent::SizeChanged(w, h) => {
                    self.width = *w as u32;
                    self.height = *h as u32;
                    self.chess_game.write().textures.render_board(
                        self.canvas.clone(),
                        (self.width as u32, self.height as u32),
                        &self.chess_game.read().board,
                    )?;
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}
