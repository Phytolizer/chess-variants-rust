#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gfx::Button;
use gfx::Widgety;

use sdl2::event::Event::Quit;
use sdl2::event::Event::RenderTargetsReset;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::render::Canvas;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl_error::{SdlError, ToSdl};

use std::fs;

mod chess_game;
mod gfx;
mod sdl_error;

fn render_texture(t: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), SdlError> {
    canvas.with_texture_canvas(t, |c| {
        c.set_draw_color(Color::RED);
        c.clear();
    })?;
    Ok(())
}

fn generate_piece_factory_from_files<'tc>(
    path: String,
    settings: &mut chess::ChessSettings<'tc>,
    texture_creator: &'tc TextureCreator<WindowContext>,
) -> Result<(), Error> {
    let dir = fs::read_dir(path)?;
    settings.factory.clear();
    for file in dir {
        let file = file?;
        if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
            settings
                .factory
                .push(chess::PieceFactory::new(&file, texture_creator)?);
        }
    }
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

        let mut chess_game: chess::Chess<WindowContext> = chess::Chess::new(
            &texture_creator,
            canvas.window().size().0,
            canvas.window().size().1,
        )
        .sdl_error()?;
        chess_game.grid.redraw(
            canvas.window().size().0,
            canvas.window().size().1,
            &mut chess_game.settings,
            &mut canvas,
        )?;
        generate_piece_factory_from_files(
            "./chess_pieces".to_string(),
            &mut chess_game.settings,
            &texture_creator,
        )?;
        chess_game.generate_pieces();

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
                        chess_game.grid.redraw(
                            canvas.window().size().0,
                            canvas.window().size().1,
                            &mut chess_game.settings,
                            &mut canvas,
                        )?;
                    }
                    _ => {}
                }
                test_button.handle_event(e)?;
            }

            canvas.set_draw_color(Color::RGB(0x20, 0x20, 0x20));
            canvas.clear();
            canvas
                .copy(
                    &chess_game.grid.texture,
                    None,
                    Rect::new(
                        chess_game.grid.off_horz,
                        chess_game.grid.off_vert,
                        chess_game.grid.size_horz,
                        chess_game.grid.size_vert,
                    ),
                )
                .sdl_error()?;
            test_button.draw(&mut canvas)?;
            chess_game.display_pieces(&mut canvas)?;
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
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
