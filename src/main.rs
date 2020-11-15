#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chess_game;
mod gfx;
mod sdl_error;

use sdl2::{
    event::Event::{Quit, RenderTargetsReset},
    pixels::Color,
    render::{BlendMode, Canvas, TargetRenderError, Texture, TextureValueError},
    video::Window,
};
use sdl_error::{SdlError, ToSdl};

use gfx::{Button, Widgety};

fn render_texture(t: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), SdlError> {
    canvas.with_texture_canvas(t, |c| {
        c.set_draw_color(Color::RED);
        c.clear();
    })?;
    Ok(())
}

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

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .target_texture()
            .build()
            .sdl_error()?;
        canvas.set_blend_mode(BlendMode::Blend);

        let texture_creator = canvas.texture_creator();

        let mut test_texture = texture_creator
            .create_texture_target(None, 100, 100)
            .sdl_error()?;

        render_texture(&mut test_texture, &mut canvas)?;

        let mut event_pump = sdl.event_pump().sdl_error()?;

        let mut width = 800u32;
        let mut height = 600u32;

        let mut chess_game = chess_game::ChessGame::new(&texture_creator)?;
        chess_game.load()?;
        chess_game
            .textures
            .render_board(&mut canvas, (width, height), &chess_game.board)?;

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

        'run: loop {
            for e in event_pump.poll_iter() {
                match e {
                    Quit { .. } => break 'run,
                    RenderTargetsReset { .. } => {
                        render_texture(&mut test_texture, &mut canvas)?;
                        chess_game.textures.render_board(
                            &mut canvas,
                            (width, height),
                            &chess_game.board,
                        )?;
                    }
                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        sdl2::event::WindowEvent::SizeChanged(w, h) => {
                            width = w as u32;
                            height = h as u32;
                            chess_game.textures.render_board(
                                &mut canvas,
                                (width, height),
                                &chess_game.board,
                            )?;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                test_button.handle_event(e)?;
            }

            canvas.set_draw_color(Color::RGB(0x20, 0x20, 0x20));
            canvas.clear();
            chess_game.textures.render(&mut canvas, &chess_game.board)?;
            test_button.draw(&mut canvas)?;
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
