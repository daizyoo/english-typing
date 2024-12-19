use crossterm::style::Color;

use crate::utils::print;

#[derive(Clone)]
pub struct Char {
    c: char,
    status: bool,
}

impl Char {
    pub fn new(c: char, status: bool) -> Self {
        Self { c, status }
    }

    pub fn print(&self) {
        let color = if self.status {
            Color::Green
        } else {
            Color::Red
        };
        print(&self.c.to_string(), color);
    }
    pub fn c(&self) -> char {
        self.c
    }
}

impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        self.c == *other
    }
}

impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}
