#![windows_subsystem = "windows"]

use sdl2::event::Event::Quit;
use sdl2::event::Event::RenderTargetsReset;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::{io::Read, path::Path};

mod chess;
mod gfx;

fn render_texture(t: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
    canvas.with_texture_canvas(t, |c| {
        c.set_draw_color(Color::RED);
        c.clear();
    })?;
    Ok(())
}

fn file_to_string<P: AsRef<Path>>(file_name: P) -> io::Result<String> {
    let mut f = fs::File::open(file_name)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn file_to_lines<P: AsRef<Path>>(file_name: P) -> io::Result<Vec<String>> {
    let f = fs::File::open(file_name)?;
    let reader = BufReader::new(f);
    let mut err: io::Result<()> = Ok(());
    let contents: Vec<String> = reader
        .lines()
        .map(|l| match l {
            Ok(l) => l.to_owned(),
            Err(e) => {
                err = Err(e);
                String::new()
            }
        })
        .collect();
    err?;
    Ok(contents)
}

fn read_files() -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir("./chess/pieces")?;
    for file in dir {
        let file = file?;
        if file.file_type()?.is_file() {
            for line in file_to_lines(file.path())? {}
        }
    }
    return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let sdl_video = sdl.video()?;
    let window = sdl_video
        .window("Test window", 800, 600)
        .position_centered()
        .vulkan()
        .resizable()
        .build()?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build()?;

    let texture_creator = canvas.texture_creator();

    let mut test_texture = texture_creator.create_texture_target(None, 100, 100)?;

    render_texture(&mut test_texture, &mut canvas)?;

    let mut event_pump = sdl.event_pump()?;

    let mut chess_game: chess::Chess<WindowContext> = chess::Chess::new(
        &texture_creator,
        canvas.window().size().0,
        canvas.window().size().1,
    )?;
    chess_game.grid.redraw(
        canvas.window().size().0,
        canvas.window().size().1,
        &mut chess_game.settings,
        &mut canvas,
    )?;

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
        }

        canvas.set_draw_color(Color::RGB(0x20, 0x20, 0x20));
        canvas.clear();
        canvas.copy(
            &chess_game.grid.texture,
            None,
            Rect::new(
                chess_game.grid.off_horz,
                chess_game.grid.off_vert,
                chess_game.grid.size_horz,
                chess_game.grid.size_vert,
            ),
        )?;
        canvas.present();
    }

    return Ok(());
}
