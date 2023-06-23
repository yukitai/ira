use std::fmt::Display;

use scratch_loader::sb3::Sb3File;

use crate::ast::ParsedScratchProject;

#[derive(Debug)]
pub enum ParseSb3Error {
    Unsupported(String),
}

impl Display for ParseSb3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseSb3Error::Unsupported(msg) => write!(f, "unsupported: {}", msg),
        }
    }
}

pub struct Sb3FormatParser {
    src: Sb3File,
}

impl Sb3FormatParser {
    pub fn new(src: Sb3File) -> Self {
        Self { src }
    }

    pub fn parse(self) -> Result<ParsedScratchProject, ParseSb3Error> {
        Err(ParseSb3Error::Unsupported(format!(
            "the sb3 file format parser hasn't been implemented'"
        )))
    }
}
