#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess_game;
mod events;
mod gfx;

use events::EventHandler;
use parking_lot::RwLock;
use sdl2::event::Event::Quit;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;
use sdl2::render::TargetRenderError;
use sdl2::render::TextureValueError;
use sdl_helpers::SdlError;
use std::rc::Rc;

use gfx::Button;

fn main() {
    let result = (|| -> Result<(), Error> {
        let sdl = sdl2::init().map_err(SdlError::Init)?;
        let video = sdl.video().map_err(SdlError::InitVideo)?;
        let mut canvas = video
            .window("Test window", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .map_err(SdlError::CreateWindow)?
            .into_canvas()
            .accelerated()
            .present_vsync()
            .target_texture()
            .build()
            .map_err(SdlError::CreateCanvas)?;
        canvas.set_blend_mode(BlendMode::Blend);

        let texture_creator = canvas.texture_creator();

        let mut event_pump = sdl.event_pump().map_err(SdlError::EventPump)?;

        let width = 800u32;
        let height = 600u32;

        let chess_game = Rc::new(RwLock::new(chess_game::ChessGame::new(&texture_creator)?));
        chess_game.write().load()?;
        chess_game
            .write()
            .render_board(&mut canvas, width, height)?;

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
        let test_button = test_button.build();
        let mut event_handler = EventHandler::new(
            chess_game.clone(),
            vec![Box::new(test_button)],
            width,
            height,
        );

        'run: loop {
            for e in event_pump.poll_iter() {
                if let Quit { .. } = e {
                    break 'run;
                }
                event_handler.handle_event(&e, &mut canvas)?;
            }

            canvas.set_draw_color(Color::RGB(0x20, 0x20, 0x20));
            canvas.clear();
            chess_game
                .read()
                .textures
                .render(&mut canvas, &chess_game.read().board)?;
            event_handler.draw_widgets(&mut canvas)?;
            canvas.present();
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
