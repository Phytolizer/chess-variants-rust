use super::piece::Piece;
use lazy_static::lazy_static;
use regex::Regex;
use sdl2::{image::LoadSurface, surface::Surface};
use sdl2::{
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{io::BufRead, num::ParseIntError};

lazy_static! {
    static ref TXT_FILE_REGEX: Regex = Regex::new(r"\.txt$").unwrap();
}

pub struct PieceFactory<'tc> {
    pub piece_name: String,
    pub piece_movement: Vec<Vec<i32>>,
    // FIXME not an option
    pub texture: Option<Texture<'tc>>,
    texture_creator: TextureCreator<WindowContext>,
}

enum State {
    Start,
    Name,
    LFMove,
    Move,
}

impl<'tc> PieceFactory<'tc> {
    pub fn new(
        file: std::fs::DirEntry,
        texture_creator: TextureCreator<WindowContext>,
    ) -> Result<PieceFactory<'tc>, Box<dyn std::error::Error>> {
        let mut piece_name = String::new();
        let mut piece_movement: Vec<Vec<i32>> = vec![];
        let mut state = State::Start;
        for line in file_to_lines(file.path())? {
            use State::*;
            if line.trim().is_empty() || line.starts_with("#") {
                continue;
            }
            match state {
                Start => {
                    if line == "start_name" {
                        state = Name;
                    }
                }
                Name => {
                    piece_name = line;
                    state = LFMove;
                }
                LFMove => {
                    if line == "start_moves" {
                        state = Move;
                    }
                }
                Move => {
                    if line == "end_moves" {
                        break;
                    }
                    let parts: Result<Vec<i32>, ParseIntError> =
                        line.split_whitespace().take(3).map(|n| n.parse()).collect();
                    let parts = parts?;
                    piece_movement.push(parts);
                }
            }
        }
        // TODO make this work!
        // let image_surface = Surface::from_file(
        //     TXT_FILE_REGEX
        //         .replacen(&file.file_name().to_string_lossy(), 1, ".png")
        //         .to_string(),
        // )?;
        // let texture = texture_creator.create_texture_from_surface(image_surface)?;

        return Ok(PieceFactory {
            piece_name,
            piece_movement,
            // FIXME texture should never be None
            texture: None,
            texture_creator,
        });
    }
    pub fn build_piece(&mut self, team: u32, pos_horz: u32, pos_vert: u32) -> Piece {
        let new_piece: Piece = Piece::new(team, pos_horz, pos_vert);
        return new_piece;
    }
}

fn file_to_lines<P: AsRef<std::path::Path>>(file_name: P) -> std::io::Result<Vec<String>> {
    let f = std::fs::File::open(file_name)?;
    let reader = std::io::BufReader::new(f);
    let mut err: std::io::Result<()> = Ok(());
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
