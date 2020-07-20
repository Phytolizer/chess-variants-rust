mod piece;
use piece::Piece;

mod piece_factory;
use piece_factory::Factory;

use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::render::TextureCreator;
use sdl2::render::TextureValueError;

pub enum GameType {
    Classic,
    Random,
}

pub struct Chess<'tc, T> {
    pub settings: ChessSettings,
    pub pieces: Vec<Piece>,
    pub grid: ChessGrid<'tc, T>,
    pub player_turn: u32,
    pub selected_piece: u32,
}
pub struct ChessSettings {
    pub game_type: GameType,
    pub squares_horz: u32,
    pub squares_vert: u32,
    pub squares_size: u32,
    pub starting_rows: u32,
    pub game_teams: u32,
    pub factory: Factory,
}
pub struct ChessGrid<'tc, T> {
    pub texture: Texture<'tc>,
    texture_creator: &'tc TextureCreator<T>,
    pub size_horz: u32,
    pub size_vert: u32,
    pub off_horz: i32,
    pub off_vert: i32,
}

impl<'tc, T> Chess<'tc, T> {
    pub fn new(
        texture_creator: &'tc TextureCreator<T>,
        width: u32,
        height: u32,
    ) -> Result<Chess<'tc, T>, TextureValueError>
    where
        T: 'tc,
    {
        Ok(Chess {
            settings: ChessSettings::new(),
            grid: ChessGrid::new(texture_creator, width, height)?,
            pieces: vec![],
            player_turn: 0,
            selected_piece: 0,
        })
    }

    pub fn generate_pieces(&mut self) {
        self.pieces.clear();
        match self.settings.game_type {
            GameType::Classic => generate_classic(&mut self.settings),
            GameType::Random => self.pieces = generate_random(&mut self.settings),
            _ => return,
        }
    }
}

pub fn generate_classic(settings: &mut ChessSettings) {}
pub fn generate_random(settings: &mut ChessSettings) -> Vec<Piece> {
    let new_pieces = vec![];
    /*for row in 0..settings.starting_rows {
        for col in 0..settings.squares_horz {
            settings
                .factory
                .build_piece(0, "Rook", settings.squares_vert - row - 1, col);
            settings.factory.build_piece(1, "Rook", row, col);
        }
    }*/
    return new_pieces;
}

impl ChessSettings {
    pub fn new() -> ChessSettings {
        ChessSettings {
            game_type: GameType::Classic,
            squares_horz: 8,
            squares_vert: 8,
            squares_size: 0,
            starting_rows: 2,
            game_teams: 2,
            factory: Factory::new(),
        }
    }
}

impl<'tc, T> ChessGrid<'tc, T> {
    pub fn new(
        texture_creator: &'tc TextureCreator<T>,
        width: u32,
        height: u32,
    ) -> Result<ChessGrid<'tc, T>, TextureValueError>
    where
        T: 'tc,
    {
        let texture = texture_creator.create_texture(None, TextureAccess::Target, width, height)?;
        Ok(ChessGrid {
            texture,
            texture_creator,
            size_horz: 0,
            size_vert: 0,
            off_horz: 0,
            off_vert: 0,
        })
    }

    pub fn redraw<RT>(
        &mut self,
        width: u32,
        height: u32,
        settings: &mut ChessSettings,
        canvas: &mut Canvas<RT>,
    ) -> Result<(), Box<dyn Error>>
    where
        RT: RenderTarget,
    {
        let squares_size: u32;
        if width / settings.squares_horz < height / settings.squares_vert {
            squares_size = width / settings.squares_horz;
        } else {
            squares_size = height / settings.squares_vert;
        }

        settings.squares_size = squares_size;
        self.size_horz = settings.squares_horz * squares_size;
        self.size_vert = settings.squares_vert * squares_size;
        self.off_horz = ((width - self.size_horz) / 2) as i32;
        self.off_vert = ((height - self.size_vert) / 2) as i32;

        self.texture = self.texture_creator.create_texture(
            None,
            TextureAccess::Target,
            self.size_horz,
            self.size_vert,
        )?;

        canvas.with_texture_canvas(&mut self.texture, |canvas: &mut Canvas<RT>| {
            canvas.set_draw_color(Color::RGB(0x80, 0x80, 0x80));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(0x40, 0x40, 0x40));
            for horz in 0..settings.squares_horz {
                for vert in 0..settings.squares_vert {
                    if horz % 2 == vert % 2 {
                        canvas
                            .fill_rect(Rect::new(
                                (horz * squares_size) as i32,
                                (vert * squares_size) as i32,
                                squares_size,
                                squares_size,
                            ))
                            .unwrap();
                    }
                }
            }
        })?;

        Ok(())
    }
}
