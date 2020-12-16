use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::{BufReader, Read},
    iter::Peekable,
};

use super::piece_move::MoveRules;
use super::InvalidFormatError;
use super::{piece::Piece, piece_move::PieceMove};

#[derive(Debug)]
pub struct PieceCatalog {
    pub catalog: HashMap<String, Piece>,
}

#[derive(Debug, PartialEq)]
enum PieceTokenKind {
    NameKeyword,
    ImageKeyword,
    LeapKeyword,
    KillKeyword,
    SpecialKeyword,
    RunKeyword,

    Colon,
    Number(i32),
    Text(String),

    EndOfFile,
}

#[derive(Debug)]
struct PieceToken {
    line: usize,
    text: String,
    kind: PieceTokenKind,
}

#[derive(Debug)]
enum PieceStatement {
    Name {
        name: String,
    },
    Image {
        image_path: String,
    },
    Move {
        kind: MoveRules,
        forward: i32,
        left: i32,
    },
    SpecialMove {
        kind: MoveRules,
    },
}

impl PieceCatalog {
    pub fn new() -> Result<PieceCatalog, crate::Error> {
        Ok(PieceCatalog {
            catalog: HashMap::new(),
        })
    }

    pub fn generate(&mut self, dir_path: String) -> Result<(), crate::Error> {
        let dir = fs::read_dir(dir_path)?;
        for path in dir {
            let file = path?;
            if file.file_type()?.is_file() && file.file_name().to_string_lossy().ends_with(".txt") {
                let reader = BufReader::new(File::open(file.path())?);
                let piece = Self::read_piece(reader)?;
                self.catalog.insert(piece.name.clone(), piece);
            }
        }
        Ok(())
    }

    fn lex_piece<R: Read>(reader: R) -> Result<Vec<PieceToken>, crate::Error> {
        let mut reader = reader.bytes().peekable();
        let mut line = 1;
        let mut tokens = Vec::<PieceToken>::new();

        while let Some(b) = reader.next() {
            let b = b?;

            match b {
                b'\n' => line += 1,
                b'\t' | b' ' | b'\r' => {}
                b':' => tokens.push(PieceToken {
                    line,
                    text: String::from(":"),
                    kind: PieceTokenKind::Colon,
                }),
                b'-' => {
                    while let Some(Ok(b'-')) = reader.peek() {
                        reader.next();
                    }
                    if let Some(&Ok(b)) = reader.peek() {
                        if b.is_ascii_digit() {
                            reader.next();
                            let text = format!("-{}", b as char);
                            tokens.push(PieceToken {
                                line,
                                kind: PieceTokenKind::Number(text.parse()?),
                                text,
                            })
                        }
                    }
                }
                b if b.is_ascii_digit() => {
                    let text = (b as char).to_string();
                    tokens.push(PieceToken {
                        line,
                        kind: PieceTokenKind::Number(text.parse()?),
                        text,
                    });
                }
                b if b.is_ascii_alphabetic() => {
                    let mut word = (b as char).to_string();
                    while let Some(&Ok(mut b)) = reader.peek() {
                        if b.is_ascii_alphabetic() || b == b'.' || b == b'_' {
                            if b == b'_' {
                                b = b' ';
                            }
                            word.push(b as char);
                            reader.next();
                        } else {
                            break;
                        }
                    }
                    let kind = match word.as_str() {
                        "Name" => PieceTokenKind::NameKeyword,
                        "Image" => PieceTokenKind::ImageKeyword,
                        "Run" => PieceTokenKind::RunKeyword,
                        "Leap" => PieceTokenKind::LeapKeyword,
                        "Kill" => PieceTokenKind::KillKeyword,
                        "Special" => PieceTokenKind::SpecialKeyword,
                        _ => PieceTokenKind::Text(word.clone()),
                    };
                    tokens.push(PieceToken {
                        line,
                        kind,
                        text: word,
                    })
                }
                _ => return Err(InvalidFormatError::new(line, (b as char).to_string()).into()),
            }
        }
        tokens.push(PieceToken {
            kind: PieceTokenKind::EndOfFile,
            line,
            text: String::new(),
        });
        Ok(tokens)
    }

    fn piece_name_statement(
        tokens: &mut Peekable<impl Iterator<Item = PieceToken>>,
    ) -> Result<PieceStatement, crate::Error> {
        tokens.next();
        let colon = tokens.next().unwrap();
        if colon.kind != PieceTokenKind::Colon {
            return Err(InvalidFormatError::new(colon.line, colon.text).into());
        }
        let name = tokens.next().unwrap();
        match name.kind {
            PieceTokenKind::Text(name) => Ok(PieceStatement::Name { name }),
            _ => Err(InvalidFormatError::new(name.line, name.text).into()),
        }
    }

    fn piece_image_statement(
        tokens: &mut Peekable<impl Iterator<Item = PieceToken>>,
    ) -> Result<PieceStatement, crate::Error> {
        tokens.next();
        let colon = tokens.next().unwrap();
        if colon.kind != PieceTokenKind::Colon {
            return Err(InvalidFormatError::new(colon.line, colon.text).into());
        }
        let image = tokens.next().unwrap();
        match image.kind {
            PieceTokenKind::Text(image_path) => Ok(PieceStatement::Image { image_path }),
            _ => Err(InvalidFormatError::new(image.line, image.text).into()),
        }
    }

    fn piece_move_statement(
        tokens: &mut Peekable<impl Iterator<Item = PieceToken>>,
    ) -> Result<PieceStatement, crate::Error> {
        let move_token = tokens.next().unwrap();
        let move_kind = match move_token.kind {
            PieceTokenKind::KillKeyword => MoveRules::Kill,
            PieceTokenKind::LeapKeyword => MoveRules::Leap,
            PieceTokenKind::RunKeyword => MoveRules::Run,
            _ => unreachable!(),
        };
        let colon = tokens.next().unwrap();
        if colon.kind != PieceTokenKind::Colon {
            return Err(InvalidFormatError::new(colon.line, colon.text).into());
        }
        let forward_token = tokens.next().unwrap();
        let forward = match forward_token.kind {
            PieceTokenKind::Number(f) => f,
            _ => {
                return Err(InvalidFormatError::new(forward_token.line, forward_token.text).into())
            }
        };
        let left_token = tokens.next().unwrap();
        let left = match left_token.kind {
            PieceTokenKind::Number(l) => l,
            _ => return Err(InvalidFormatError::new(left_token.line, left_token.text).into()),
        };
        Ok(PieceStatement::Move {
            kind: move_kind,
            forward,
            left,
        })
    }

    fn piece_special_move_statement(
        tokens: &mut Peekable<impl Iterator<Item = PieceToken>>,
    ) -> Result<PieceStatement, crate::Error> {
        tokens.next();
        let colon = tokens.next().unwrap();
        if colon.kind != PieceTokenKind::Colon {
            return Err(InvalidFormatError::new(colon.line, colon.text).into());
        }
        let special_token = tokens.next().unwrap();
        let special = match special_token.kind {
            PieceTokenKind::Text(special) => special,
            _ => {
                return Err(InvalidFormatError::new(special_token.line, special_token.text).into())
            }
        };
        Ok(PieceStatement::SpecialMove {
            kind: match special.as_str() {
                "Castle" => MoveRules::Castle,
                "PawnFirst" => MoveRules::PawnFirst,
                _ => {
                    return Err(
                        InvalidFormatError::new(special_token.line, special_token.text).into(),
                    )
                }
            },
        })
    }

    fn piece_statement(
        tokens: &mut Peekable<impl Iterator<Item = PieceToken>>,
    ) -> Result<PieceStatement, crate::Error> {
        match tokens.peek().unwrap().kind {
            PieceTokenKind::NameKeyword => Self::piece_name_statement(tokens),
            PieceTokenKind::ImageKeyword => Self::piece_image_statement(tokens),
            PieceTokenKind::LeapKeyword
            | PieceTokenKind::KillKeyword
            | PieceTokenKind::RunKeyword => Self::piece_move_statement(tokens),
            PieceTokenKind::SpecialKeyword => Self::piece_special_move_statement(tokens),
            _ => Err(InvalidFormatError::new(
                tokens.peek().unwrap().line,
                tokens.peek().unwrap().text.clone(),
            )
            .into()),
        }
    }

    fn parse_piece(tokens: impl Iterator<Item = PieceToken>) -> Result<Piece, crate::Error> {
        let mut tokens = tokens.peekable();
        let mut statements = Vec::<PieceStatement>::new();
        while tokens.peek().unwrap().kind != PieceTokenKind::EndOfFile {
            statements.push(Self::piece_statement(&mut tokens)?);
        }
        let mut piece = Piece::new();
        dbg!(&statements);
        for statement in statements {
            match statement {
                PieceStatement::Name { name } => piece.name = name,
                PieceStatement::Image { image_path } => piece.image_key = image_path,
                PieceStatement::Move {
                    kind,
                    forward,
                    left,
                } => piece.move_set.push(PieceMove::new(forward, left, kind)),
                PieceStatement::SpecialMove { kind } => {
                    piece.move_set.push(PieceMove::new_special(kind))
                }
            }
        }
        Ok(piece)
    }

    fn read_piece<R: Read>(reader: R) -> Result<Piece, crate::Error> {
        let tokens = Self::lex_piece(reader)?;
        Self::parse_piece(tokens.into_iter())
    }

    pub fn get_piece(&self, piece_name: &str) -> Result<&Piece, crate::Error> {
        let out = self
            .catalog
            .get(piece_name)
            .ok_or_else(|| PieceNotFoundError {
                name: piece_name.to_string(),
            })?;
        Ok(out)
    }
}

#[derive(Debug)]
pub struct PieceNotFoundError {
    name: String,
}

impl Display for PieceNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Piece not found: {}", self.name)
    }
}

impl std::error::Error for PieceNotFoundError {}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(expected: Expect, actual: String) {
        expected.assert_eq(&actual);
    }

    #[test]
    fn lex_stuff() {
        let data = "
            Name: King
            ";
        let tokens = PieceCatalog::lex_piece(data.as_bytes()).unwrap();
        check(
            expect![[r#"
                [
                    PieceToken {
                        line: 2,
                        text: "Name",
                        kind: NameKeyword,
                    },
                    PieceToken {
                        line: 2,
                        text: ":",
                        kind: Colon,
                    },
                    PieceToken {
                        line: 2,
                        text: "King",
                        kind: Text(
                            "King",
                        ),
                    },
                    PieceToken {
                        line: 3,
                        text: "",
                        kind: EndOfFile,
                    },
                ]"#]],
            format!("{:#?}", tokens),
        );
    }

    #[test]
    fn parse_name_statement() {
        let data = "Name: King";
        let piece = PieceCatalog::parse_piece(
            PieceCatalog::lex_piece(data.as_bytes())
                .unwrap()
                .into_iter(),
        )
        .unwrap();
        check(
            expect![[r#"
            Piece {
                name: "King",
                image_key: "",
                move_set: [],
                promotions: [],
            }"#]],
            format!("{:#?}", piece),
        );
    }

    #[test]
    fn parse_move_statement() {
        let data = "Leap: -1 1";
        let tokens = PieceCatalog::lex_piece(data.as_bytes()).unwrap();
        let piece = PieceCatalog::parse_piece(tokens.into_iter()).unwrap();
    }

    #[test]
    fn read_basic_piece() {
        let data = "
            ----------------------------
            Name: King
            Image: King.png
            ----------------------------
            Leap: 1 1
            Leap: 1 -1
            Leap: -1 1
            Leap: -1 -1
            Leap: 1 1
            Leap: 1 -1
            Leap: -1 1
            Leap: -1 -1
            ----------------------------
            Kill: 1 1
            Kill: 1 -1
            Kill: -1 1
            Kill: -1 -1
            Kill: 1 1
            Kill: 1 -1
            Kill: -1 1
            Kill: -1 -1
            ----------------------------
            Special: Castle
            ----------------------------
        ";
        let piece = PieceCatalog::read_piece(data.as_bytes()).unwrap();
        check(
            expect![[r#"
                Piece {
                    name: "King",
                    image_key: "King.png",
                    move_set: [
                        PieceMove {
                            forward: 1,
                            left: 1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: 1,
                            left: -1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: -1,
                            left: 1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: -1,
                            left: -1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: 1,
                            left: 1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: 1,
                            left: -1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: -1,
                            left: 1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: -1,
                            left: -1,
                            rules: Leap,
                        },
                        PieceMove {
                            forward: 1,
                            left: 1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: 1,
                            left: -1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: -1,
                            left: 1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: -1,
                            left: -1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: 1,
                            left: 1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: 1,
                            left: -1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: -1,
                            left: 1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: -1,
                            left: -1,
                            rules: Kill,
                        },
                        PieceMove {
                            forward: 0,
                            left: 0,
                            rules: Castle,
                        },
                    ],
                    promotions: [],
                }"#]],
            format!("{:#?}", piece),
        );
    }

    #[test]
    fn name_with_space() {
        let data = "Name: King_Killer";
        let tokens = PieceCatalog::lex_piece(data.as_bytes()).unwrap();
        let piece = PieceCatalog::parse_piece(tokens.into_iter()).unwrap();
        check(expect![[r#"
            Piece {
                name: "King Killer",
                image_key: "",
                move_set: [],
                promotions: [],
            }"#]], format!("{:#?}", piece));
    }
}
