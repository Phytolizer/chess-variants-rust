use parking_lot::RwLock;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl_helpers::SdlError;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use super::board::Board;

pub struct TextureRegistry<'tc, C> {
    pub texture_creator: &'tc TextureCreator<C>,
    pub board_texture: Option<Texture<'tc>>,
    pub pieces: HashMap<String, Texture<'tc>>,
}

impl<'tc, C> TextureRegistry<'tc, C> {
    pub fn new(texture_creator: &'tc TextureCreator<C>) -> Self {
        Self {
            texture_creator,
            board_texture: None,
            pieces: HashMap::new(),
        }
    }

    pub fn generate_piece_images(&mut self, dir_path: String) -> Result<(), crate::Error> {
        let dir = fs::read_dir(&dir_path)?;
        for path in dir {
            let file = path?;
            if file.file_type()?.is_file()
                && regex::Regex::new("\\.(png|jpg|jpeg)$")
                    .unwrap()
                    .is_match(&file.file_name().to_string_lossy())
            {
                let full_file_name = file.file_name().to_string_lossy().to_string();
                let tex = self
                    .texture_creator
                    .load_texture(&PathBuf::from(&dir_path).join(&full_file_name))
                    .map_err(SdlError::LoadImage)?;
                let key = full_file_name.split('.').next().unwrap();
                self.pieces.insert(key.to_string(), tex);
            }
        }
        Ok(())
    }

    pub fn render(
        &self,
        canvas: Rc<RwLock<WindowCanvas>>,
        board: &Board,
    ) -> Result<(), crate::Error> {
        board.draw(canvas.clone(), &self.board_texture)?;
        for game_piece in board.collect_game_pieces() {
            let piece_texture = match self.pieces.get(&game_piece.piece_name) {
                Some(pt) => pt,
                None => continue,
            };
            let piece_area = Rect::new(
                board.rect.x + ((game_piece.horz_position - 1) * board.space_size) as i32,
                board.rect.y + ((game_piece.vert_position - 1) * board.space_size) as i32,
                board.space_size,
                board.space_size,
            );
            canvas
                .write()
                .copy(piece_texture, None, Some(piece_area))
                .map_err(SdlError::Drawing)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UninitializedTextureRegistryError {}

impl Display for UninitializedTextureRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uninitialized texture registry")
    }
}

impl std::error::Error for UninitializedTextureRegistryError {}
