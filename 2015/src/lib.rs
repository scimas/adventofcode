use std::error::Error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct InputParseError(String);

impl Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InputParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl InputParseError {
    fn new(s: &str) -> Self {
        Self(s.to_owned())
    }
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
