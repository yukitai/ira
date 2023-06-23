mod reporter;

use scratch_loader::{load_sb3::load, sb3::Sb3File};

use reporter::Reporter;
use scratch_parser::{ast::ParsedScratchProject, parser::Sb3FormatParser};

pub fn load_sb3(src: &str) -> Sb3File {
    load(src).report()
}

pub fn parse_sb3(src: Sb3File) -> ParsedScratchProject {
    let parser = Sb3FormatParser::new(src);
    parser.parse().report()
}
