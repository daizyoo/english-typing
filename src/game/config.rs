use rand::{seq::SliceRandom, thread_rng};

use crate::level::questions;

use super::question::Question;

pub struct GameConfig {
    questions: Vec<Question>,
    level: usize,
    meaning_level: usize,
    number: u8,
}

impl GameConfig {
    const LEVEL_MAX: usize = 10;
    const MEANING_LEVEL_MAX: usize = 5;

    pub fn new(level: usize, meaning_level: usize, number: u8) -> Self {
        if level > Self::LEVEL_MAX {
            panic!("level is not found");
        }
        if meaning_level > Self::MEANING_LEVEL_MAX {
            panic!("meaning level is not found");
        }
        let mut questions = questions(level).to_vec();
        questions.shuffle(&mut thread_rng());
        Self {
            questions: questions.iter().map(|s| Question::new(s.0, s.1)).collect(),
            level,
            meaning_level,
            number,
        }
    }

    pub fn questions(&self) -> &Vec<Question> {
        &self.questions
    }
    pub fn level(&self) -> usize {
        self.level
    }
    pub fn meaning_level(&self) -> usize {
        self.meaning_level
    }
    pub fn number(&self) -> u8 {
        self.number
    }
}
