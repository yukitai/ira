use ira::{load_sb3, parse_sb3};

use colored::Colorize;

fn main() {
    // todo! parse args at first
    let proj = load_sb3(&std::env::args().nth(1).expect("usage: ira <source>"));
    let ast = parse_sb3(proj);
    println!("{}: {:?}", "result".bright_blue(), ast);
}
