mod char;
mod config;
mod game;
mod question;
mod state;
mod storage;

use std::collections::HashMap;

type WrongSections = HashMap<String, u16>;

pub use game::Game;
