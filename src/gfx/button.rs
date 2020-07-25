use super::Widget;
use super::Widgety;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use std::error::Error;

#[derive(PartialEq, Eq)]
enum State {
    Normal,
    Hovered,
    Pressed,
}

pub struct Button<CA>
where
    CA: Fn() -> Result<(), Box<dyn Error>>,
{
    pub widget: Widget,
    pub text: String,
    pub on_click: CA,

    state: State,
}

impl<CA> Widgety for Button<CA>
where
    CA: Fn() -> Result<(), Box<dyn Error>>,
{
    fn draw<RT>(&self, canvas: &mut sdl2::render::Canvas<RT>) -> Result<(), Box<dyn Error>>
    where
        RT: sdl2::render::RenderTarget,
    {
        canvas.set_draw_color(self.widget.color);
        canvas.fill_rect(self.widget.rect)?;
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_rect(self.widget.rect)?;
        match self.state {
            State::Normal => {}
            State::Hovered => {
                canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0x20));
                canvas.fill_rect(self.widget.rect)?;
            }
            State::Pressed => {
                canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0x40));
                canvas.fill_rect(self.widget.rect)?;
            }
        }
        Ok(())
    }
    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::MouseMotion { x, y, .. } => {
                if self.state != State::Pressed {
                    if self.widget.rect.contains_point((x, y)) {
                        if self.state == State::Normal {
                            self.state = State::Hovered;
                        }
                    } else if self.state == State::Hovered {
                        self.state = State::Normal;
                    }
                }
            }
            Event::MouseButtonDown { mouse_btn, .. } => {
                if self.state == State::Hovered && mouse_btn == MouseButton::Left {
                    self.state = State::Pressed;
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if self.state == State::Pressed && mouse_btn == MouseButton::Left {
                    if self.widget.rect.contains_point((x, y)) {
                        self.state = State::Hovered;
                        (self.on_click)()?;
                    } else {
                        self.state = State::Normal;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
