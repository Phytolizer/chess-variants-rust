#![windows_subsystem = "windows"]

use sdl2::event::Event::Quit;
use sdl2::event::Event::RenderTargetsReset;
use sdl2::pixels::Color;
use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

mod chess;

fn e_to_string<E>(e: E) -> String
where
    E: ToString,
{
    e.to_string()
}

fn render_texture(t: &mut Texture, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas
        .with_texture_canvas(t, |c| {
            c.set_draw_color(Color::RED);
            c.clear();
        })
        .map_err(e_to_string)?;

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let sdl_video = sdl.video()?;
    let window = sdl_video
        .window("Test window", 800, 600)
        .position_centered()
        .vulkan()
        .resizable()
        .build()
        .map_err(e_to_string)?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build()
        .map_err(e_to_string)?;

    let texture_creator = canvas.texture_creator();

    let mut test_texture = texture_creator
        .create_texture_target(None, 100, 100)
        .map_err(e_to_string)?;

    render_texture(&mut test_texture, &mut canvas)?;

    let mut event_pump = sdl.event_pump()?;

    let chess_game: chess::Chess = chess::Chess::new();
    let squares_horz: u32 = 8;
    let squares_vert: u32 = 8;
    let (width, height) = canvas.window().size();
    chess_game.update_grid(squares_horz, squares_vert, width, height);

    'run: loop {
        for e in event_pump.poll_iter() {
            match e {
                Quit { .. } => break 'run,
                RenderTargetsReset { .. } => {
                    render_texture(&mut test_texture, &mut canvas)?;
                    let (width, height) = canvas.window().size();
                    chess_game.update_grid(squares_horz, squares_vert, width, height);
                }
                _ => {}
            }
        }
        let (width, height) = canvas.window().size();

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.copy(&test_texture, None, Rect::new(0, 0, width / 2, height / 2))?;
        canvas.present();
    }

    return Ok(());
}
