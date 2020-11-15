use super::Widget;
use super::Widgety;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use crate::{sdl_error::ToSdl, Error};

#[derive(PartialEq, Eq)]
enum State {
    Normal,
    Hovered,
    Pressed,
}

pub struct Button {
    pub widget: Widget,
    pub text: String,
    pub on_click: fn() -> Result<(), Error>,

    state: State,
}

impl Widgety for Button {
    fn draw<RT>(&self, canvas: &mut sdl2::render::Canvas<RT>) -> Result<(), Error>
    where
        RT: sdl2::render::RenderTarget,
    {
        canvas.set_draw_color(self.widget.color);
        canvas.fill_rect(self.widget.rect).sdl_error()?;
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_rect(self.widget.rect).sdl_error()?;
        match self.state {
            State::Normal => {}
            State::Hovered => {
                canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0x20));
                canvas.fill_rect(self.widget.rect).sdl_error()?;
            }
            State::Pressed => {
                canvas.set_draw_color(Color::RGBA(0x00, 0x00, 0x00, 0x40));
                canvas.fill_rect(self.widget.rect).sdl_error()?;
            }
        }
        Ok(())
    }
    fn handle_event(&mut self, event: Event) -> Result<(), Error> {
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

pub struct ButtonBuilder {
    pub widget: Widget,
    pub text: Option<String>,
    pub on_click: fn() -> Result<(), Error>,
}

impl ButtonBuilder {
    pub fn size(&mut self, w: u32, h: u32) -> &mut Self {
        self.widget.rect.set_width(w);
        self.widget.rect.set_height(h);
        self
    }

    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.widget.rect.set_x(x);
        self.widget.rect.set_y(y);
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.widget.color = color;
        self
    }

    pub fn with_text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
        self.text = Some(text.as_ref().to_owned());
        self
    }

    pub fn with_click_action(&mut self, click_action: fn() -> Result<(), Error>) -> &mut Self {
        self.on_click = click_action;
        self
    }

    pub fn build(self) -> Button {
        Button {
            widget: self.widget.clone(),
            text: match &self.text {
                Some(text) => text.to_owned(),
                None => String::new(),
            },
            on_click: self.on_click,
            state: State::Normal,
        }
    }
}

impl Button {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ButtonBuilder {
        ButtonBuilder {
            widget: Widget::new(None),
            text: None,
            on_click: || Ok(()),
        }
    }
}
