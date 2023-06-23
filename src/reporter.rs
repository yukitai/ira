use std::fmt::Display;

use colored::Colorize;

pub fn report<T>(err: T) -> !
where
    T: Display,
{
    println!("{}: {}", "error".bright_red(), err);
    std::process::exit(1);
}

pub trait Reporter<T> {
    fn report(self) -> T;
}

impl<T, E: Display> Reporter<T> for Result<T, E> {
    fn report(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => report(err),
        }
    }
}
