#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess_game;
mod events;
mod gfx;
mod sdl_error;

use events::EventHandler;
use parking_lot::RwLock;
use sdl2::{
    event::Event::Quit,
    pixels::Color,
    render::{BlendMode, TargetRenderError, TextureValueError},
};
use sdl_error::{SdlError, ToSdl};
use std::rc::Rc;

use gfx::{Button, Widgety};

fn main() {
    let result = (|| -> Result<(), Error> {
        let sdl = sdl2::init().sdl_error()?;
        let sdl_video = sdl.video().sdl_error()?;
        let window = sdl_video
            .window("Test window", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .sdl_error()?;

        let canvas = Rc::new(RwLock::new(
            window
                .into_canvas()
                .accelerated()
                .present_vsync()
                .target_texture()
                .build()
                .sdl_error()?,
        ));
        canvas.write().set_blend_mode(BlendMode::Blend);

        let texture_creator = canvas.read().texture_creator();

        let mut event_pump = sdl.event_pump().sdl_error()?;

        let width = 800u32;
        let height = 600u32;

        let chess_game = Rc::new(RwLock::new(chess_game::ChessGame::new(&texture_creator)?));
        chess_game.write().load()?;
        chess_game.write().textures.render_board(
            canvas.clone(),
            (width, height),
            &chess_game.read().board,
        )?;

        let mut test_button = Button::new();
        test_button
            .with_text("Test button")
            .with_click_action(|| {
                println!("Hello World");
                Ok(())
            })
            .position(100, 100)
            .size(100, 100)
            .color(Color::BLUE);
        let mut test_button = test_button.build();
        let mut event_handler = EventHandler::new(
            chess_game.clone(),
            canvas.clone(),
            &[&mut test_button],
            width,
            height,
        );

        'run: loop {
            for e in event_pump.poll_iter() {
                if let Quit { .. } = e {
                    break 'run;
                }
                test_button.handle_event(&e)?;
                event_handler.handle_event(&e)?;
            }

            canvas.write().set_draw_color(Color::RGB(0x20, 0x20, 0x20));
            canvas.write().clear();
            chess_game
                .write()
                .textures
                .render(canvas.clone(), &chess_game.read().board)?;
            test_button.draw(canvas.clone())?;
            canvas.write().present();
        }

        Ok(())
    })();
    match result {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SDL error: {0}")]
    Sdl(#[from] SdlError),

    #[error(transparent)]
    TextureValue(#[from] TextureValueError),

    #[error(transparent)]
    TargetRender(#[from] TargetRenderError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    InvalidFileFormat(#[from] chess_game::InvalidFormatError),

    #[error(transparent)]
    PieceNotFound(#[from] chess_game::piece_catalog::PieceNotFoundError),

    #[error(transparent)]
    UninitializedTextureRegistry(
        #[from] chess_game::texture_registry::UninitializedTextureRegistryError,
    ),
}
