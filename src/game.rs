use std::{
    collections::HashMap,
    fs,
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::style::Color;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    gemini::Gemini,
    level::questions,
    meaning::Meaning,
    utils::{clear, meaning_print, move_cursor, print},
};

type WrongSections = HashMap<String, u16>;

pub struct Game {
    question: Question,
    input: Vec<Char>,
    now_section: String,
    sections: Vec<String>,
    storage: Storage,
    state: State,
    config: GameConfig,
}

struct GameConfig {
    questions: Vec<Question>,
    level: usize,
    meaning_level: usize,
    number: u8,
}

struct State {
    count: u8,
    correct: u8,
    time: Instant,
}

struct Storage {
    section_meanings: Vec<Vec<Meaning>>,
    level_section_meanings: Vec<Meaning>,
    inputs: Vec<Vec<Char>>,
    wrong_sections: WrongSections,
}

#[derive(Clone)]
struct Question {
    english: String,
    japanese: String,
}

#[derive(Clone)]
struct Char {
    c: char,
    status: bool,
}

impl Question {
    fn new(english: &'static str, japanese: &'static str) -> Self {
        Self {
            english: english.to_string(),
            japanese: japanese.to_string(),
        }
    }
}

impl Storage {
    const MEANING_FILE: &str = "meaning.json";

    fn new(meaning_level: usize) -> Self {
        let section_meanings = fs::read_to_string(Self::MEANING_FILE).unwrap();
        let meanings: Vec<Vec<Meaning>> = serde_json::from_str(&section_meanings).unwrap();
        Self {
            level_section_meanings: meanings[meaning_level - 1].clone(),
            section_meanings: meanings,
            inputs: Vec::new(),
            wrong_sections: WrongSections::new(),
        }
    }
    fn push(&mut self, input: Vec<Char>) {
        self.inputs.push(input);
    }
    fn print_inputs(&self) {
        for input in self.inputs.iter() {
            for c in input.iter() {
                c.print();
            }
            println!();
        }
    }
    fn save_meanings(&self) {
        fs::write(
            Self::MEANING_FILE,
            serde_json::to_string(&self.section_meanings).unwrap(),
        )
        .unwrap();
    }
    async fn show_meanings(&mut self, meaning_level: usize) {
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
}

impl Char {
    fn new(c: char, status: bool) -> Self {
        Self { c, status }
    }

    fn print(&self) {
        let color = if self.status {
            Color::Green
        } else {
            Color::Red
        };
        print(&self.c.to_string(), color);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            count: 0,
            correct: 0,
            time: Instant::now(),
        }
    }
}

impl GameConfig {
    const LEVEL_MAX: usize = 10;
    const MEANING_LEVEL_MAX: usize = 5;

    fn new(level: usize, meaning_level: usize, number: u8) -> Self {
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
}

impl Game {
    pub fn new(level: usize, meaning_level: usize, number: u8) -> Self {
        let config = GameConfig::new(level, meaning_level, number);
        let question = config.questions[0].clone();
        let mut game = Self {
            question: question.clone(),
            input: Vec::new(),
            now_section: String::new(),
            sections: question.english.split(' ').map(|s| s.to_string()).collect(),
            storage: Storage::new(meaning_level),
            state: State::default(),
            config,
        };
        game.change_section();
        game
    }
    pub fn draw(&self) {
        clear();
        println!("{}", self.question.english);
        println!("{}", self.question.japanese);
        println!("{}", self.now_section);
        println!("{:?}", self.sections);
        println!("{:?}", self.storage.wrong_sections);
        move_cursor(0, 0);
        self.input.iter().for_each(|c| c.print());
    }
    pub fn set_sections(&mut self) {
        self.sections = self
            .question
            .english
            .split(' ')
            .map(|s| s.to_string().to_lowercase())
            .collect();
    }
    pub fn change_section(&mut self) {
        let mut section_len = 0;
        for section in self.sections.iter() {
            section_len += section.len() + 1;
            if self.input.len() < section_len {
                self.now_section = section.clone();
                break;
            }
        }
    }
    /// 問題を変更
    fn change_question(&mut self) {
        self.question = self.config.questions[self.state.count as usize].clone();
    }
    /// 入力
    pub fn input(&mut self, c: char) {
        // 入力結果が正しいかどうか
        if self.question.english.chars().nth(self.input.len()).unwrap() == c {
            self.input.push(Char::new(c, true));
        } else {
            self.input.push(Char::new(c, false));
            self.wrong_input();
        }
    }
    fn wrong_input(&mut self) {
        let wrong_section: String = self
            .now_section
            .split(|c| [',', '.'].contains(&c))
            .filter(|s| !s.is_empty())
            .collect();
        if let Some(count) = self.storage.wrong_sections.get_mut(&wrong_section) {
            *count += 1;
        } else {
            self.storage.wrong_sections.insert(wrong_section, 1);
        }
    }
    /// 入力結果を一文字削除
    pub fn delete(&mut self) {
        self.input.pop();
    }
    /// 次の問題へ
    pub fn next_question(&mut self) {
        self.state.count += 1; // 問題数をカウント
        self.input = Vec::new(); // 入力結果をリセット
        self.push_storage(); // 入力結果を保存
        self.change_question(); // 問題を変更
        self.set_sections(); // 単語リストを変更
        self.change_section(); // 現在の単語を変更
    }
    /// 入力結果を保存
    fn push_storage(&mut self) {
        self.storage.push(self.input.clone());
    }
    /// 入力結果が正しいかどうか
    pub fn is_correct(&self) -> bool {
        self.input.iter().map(|c| c.c).collect::<String>() == self.question.english
    }
    /// 正回数をカウント
    pub fn correct_count(&mut self) {
        self.state.correct += 1;
    }
    /// 入力が終了したか
    pub fn is_input_end(&self) -> bool {
        self.input.len() == self.question.english.len()
    }
    /// ゲームが終了したか
    pub fn is_game_end(&self) -> bool {
        self.state.count >= self.config.number
    }
    /// ゲーム終了時の処理
    pub fn quit(&self) {
        clear();
        self.storage.print_inputs();
        println!("level: {}", self.config.level);
        print(
            &format!("correct: {}/{}\n", self.state.correct, self.config.number),
            Color::Green,
        );
        print(
            &self.state.time.elapsed().as_secs_f32().to_string(),
            Color::Green,
        );
        println!();
    }
    pub async fn show_meanings(&mut self) {
        self.storage.show_meanings(self.config.meaning_level).await;
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
