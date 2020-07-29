pub mod grid;
pub mod piece;
pub mod piece_factory;

pub use piece_factory::PieceFactory;

use rand::Rng;
use sdl2::render::{RenderTarget, Canvas};

pub enum GameType {
    //Classic,
    Random,
}

pub struct Chess<'tc, T> {
    pub settings: ChessSettings<'tc>,
    pub pieces: Vec<piece::Piece>,
    pub grid: grid::Grid<'tc, T>,
    pub player_turn: u32,
    pub selected_piece: u32,
}
pub struct ChessSettings<'tc> {
    pub game_type: GameType,
    pub squares_horz: u32,
    pub squares_vert: u32,
    pub squares_size: u32,
    pub starting_rows: u32,
    pub game_teams: u32,
    pub factory: Vec<piece_factory::PieceFactory<'tc>>,
}

impl<'tc, T> Chess<'tc, T> {
    pub fn new(
        texture_creator: &'tc sdl2::render::TextureCreator<T>,
        width: u32,
        height: u32,
    ) -> Result<Chess<'tc, T>, sdl2::render::TextureValueError>
    where
        T: 'tc,
    {
        Ok(Chess {
            settings: ChessSettings::new(),
            grid: grid::Grid::new(texture_creator, width, height)?,
            pieces: vec![],
            player_turn: 0,
            selected_piece: 0,
        })
    }

    pub fn generate_pieces(&mut self) {
        self.pieces.clear();
        match self.settings.game_type {
            //GameType::Classic => generate_classic(&mut self.settings),
            GameType::Random => self.pieces = generate_random(&mut self.settings),
            //_ => return,
        }
    }

    pub fn display_pieces<RT>(&mut self, canvas: &mut Canvas<RT>) where RT: RenderTarget {
        for index in 0..self.pieces.len() {
            self.pieces[index].display(canvas);
        }
    }
}

//pub fn generate_classic(settings: &mut ChessSettings) {}
pub fn generate_random(settings: &mut ChessSettings) -> Vec<piece::Piece> {
    let mut new_pieces = vec![];
    let mut rng = rand::thread_rng();
    for row in 0..settings.starting_rows {
        for col in 0..settings.squares_horz {
            let index = rng.gen_range(0, settings.factory.len());
            new_pieces.push(settings.factory[index].build_piece(
                0,
                settings.squares_vert - row - 1,
                col,
            ));
            new_pieces.push(settings.factory[index].build_piece(1, row, col));
        }
    }
    return new_pieces;
}

impl<'tc> ChessSettings<'tc> {
    pub fn new() -> ChessSettings<'tc> {
        ChessSettings {
            game_type: GameType::Random,
            squares_horz: 8,
            squares_vert: 8,
            squares_size: 0,
            starting_rows: 2,
            game_teams: 2,
            factory: vec![],
        }
    }
}
