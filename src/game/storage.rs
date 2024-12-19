use std::{fs, thread::sleep, time::Duration};

use crossterm::style::Color;

use crate::{
    gemini::Gemini,
    meaning::Meaning,
    utils::{meaning_print, print},
};

use super::{char::Char, WrongSections};

pub struct Storage {
    section_meanings: Vec<Vec<Meaning>>,
    level_section_meanings: Vec<Meaning>,
    inputs: Vec<Vec<Char>>,
    wrong_sections: WrongSections,
}

impl Storage {
    const MEANING_FILE: &str = "meaning.json";

    pub fn new(meaning_level: usize) -> Self {
        let section_meanings = fs::read_to_string(Self::MEANING_FILE).unwrap();
        let meanings: Vec<Vec<Meaning>> = serde_json::from_str(&section_meanings).unwrap();
        Self {
            level_section_meanings: meanings[meaning_level - 1].clone(),
            section_meanings: meanings,
            inputs: Vec::new(),
            wrong_sections: WrongSections::new(),
        }
    }
    pub(super) fn push(&mut self, input: Vec<Char>) {
        self.inputs.push(input);
    }
    pub fn print_inputs(&self) {
        for input in self.inputs.iter() {
            for c in input.iter() {
                c.print();
            }
            println!();
        }
    }
    pub fn save_meanings(&self) {
        fs::write(
            Self::MEANING_FILE,
            serde_json::to_string(&self.section_meanings).unwrap(),
        )
        .unwrap();
    }
    pub(super) async fn show_meanings(&mut self, meaning_level: usize) {
        println!("meaning level: {}", meaning_level);
        for (section, _) in self.wrong_sections.iter() {
            if let Some(meaning) = self
                .level_section_meanings
                .iter()
                .find(|m| m.section() == section)
            {
                println!("{}: {}", section, meaning.meaning());
                continue;
            }

            let Ok(gemini) = Gemini::default()
                .ask(Meaning::prompt(&section, meaning_level))
                .await
            else {
                continue;
            };
            let meaning = gemini.text().trim().to_string();
            let text = format!("{}\n", meaning);
            print(&section, Color::Blue);
            print!(": ");
            meaning_print(&text);
            self.level_section_meanings
                .push(Meaning::new(section.clone(), meaning));
            sleep(Duration::from_secs_f32(0.3));
        }
        self.section_meanings[meaning_level - 1] = self.level_section_meanings.clone();
        self.save_meanings();
    }
    pub fn wrong_sections(&mut self) -> &mut WrongSections {
        &mut self.wrong_sections
    }
}
