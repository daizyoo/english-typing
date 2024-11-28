mod gemini;
mod level;

use std::fs;
use std::thread::sleep;
use std::time::Duration;
use std::{collections::HashMap, io::stdout, time::Instant};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use getch_rs::Getch;
use getch_rs::Key;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use gemini::Gemini;
use level::questions;

type WrongSections = HashMap<String, u16>;

struct Game {
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
            Storage::MEANING_FILE,
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
                .find(|m| m.section == *section)
            {
                println!("{}: {}", section, meaning.meaning);
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
            typing_print(&text);
            self.level_section_meanings.push(Meaning {
                section: section.clone(),
                meaning,
            });
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
        print(
            &self.c.to_string(),
            if self.status {
                Color::Green
            } else {
                Color::Red
            },
        );
    }
}

impl State {
    fn new() -> Self {
        Self {
            count: 0,
            correct: 0,
            time: Instant::now(),
        }
    }
}

impl GameConfig {
    fn new(level: usize, meaning_level: usize, number: u8) -> Self {
        let mut questions = questions(level).to_vec();
        questions.shuffle(&mut rand::thread_rng());
        Self {
            questions: questions.iter().map(|s| Question::new(s.0, s.1)).collect(),
            level,
            meaning_level,
            number,
        }
    }
}

impl Game {
    fn new(level: usize, meaning_level: usize, number: u8) -> Self {
        let config = GameConfig::new(level, meaning_level, number);
        let question = config.questions[0].clone();
        let mut game = Self {
            question: question.clone(),
            input: Vec::new(),
            now_section: String::new(),
            sections: question.english.split(' ').map(|s| s.to_string()).collect(),
            storage: Storage::new(meaning_level),
            state: State::new(),
            config,
        };
        game.change_section();
        game
    }
    fn draw(&self) {
        clear();
        println!("{}", self.question.english);
        println!("{}", self.question.japanese);
        println!("{}", self.now_section);
        println!("{:?}", self.sections);
        println!("{:?}", self.storage.wrong_sections);
        move_cursor(0, 0);
        self.input.iter().for_each(|c| c.print());
    }
    fn set_sections(&mut self) {
        self.sections = self
            .question
            .english
            .split(' ')
            .map(|s| s.to_string().to_lowercase())
            .collect();
    }
    fn change_section(&mut self) {
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
    fn input(&mut self, c: char) {
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
    fn delete(&mut self) {
        self.input.pop();
    }
    /// 次の問題へ
    fn next_question(&mut self) {
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
    fn is_correct(&self) -> bool {
        self.input.iter().map(|c| c.c).collect::<String>() == self.question.english
    }
    /// 正回数をカウント
    fn correct_count(&mut self) {
        self.state.correct += 1;
    }
    /// 入力が終了したか
    fn is_input_end(&self) -> bool {
        self.input.len() == self.question.english.len()
    }
    /// ゲームが終了したか
    fn is_game_end(&self) -> bool {
        self.state.count >= self.config.number
    }
    /// ゲーム終了時の処理
    fn quit(&self) {
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
    async fn show_meanings(&mut self) {
        self.storage.show_meanings(self.config.meaning_level).await;
    }
}

fn typing_print(text: &str) {
    for c in text.chars() {
        print(&c.to_string(), Color::Green);
        sleep(Duration::from_secs_f32(thread_rng().gen_range(0.03..0.15)));
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    clear();
    let level = input_msg("level 1~10: ", 0);
    let meaning_level = input_msg("meaning level 1~5 (default: 1): ", 1);
    if meaning_level > 5 {
        panic!("meaning level is not found");
    }
    let end = input_msg("number of questions: ", 2);

    let mut game = Game::new(level, meaning_level, end);
    game.change_section();
    cursor_change();

    loop {
        game.draw();
        match Getch::new().getch() {
            Ok(Key::Char('\r')) => continue, // 改行は無視
            Ok(Key::Char(c)) => game.input(c),
            Ok(Key::Delete | Key::Backspace) => game.delete(),
            Ok(Key::Ctrl('c')) => break, //終了
            _ => continue,
        }
        game.change_section();
        if game.is_input_end() {
            if game.is_correct() {
                game.correct_count()
            }
            game.next_question()
        }
        if game.is_game_end() {
            break;
        }
    }
    game.quit();
    game.show_meanings().await;
    restore_cursor();
}

#[derive(Clone, Serialize, Deserialize)]
struct Meaning {
    section: String,
    meaning: String,
}

impl Meaning {
    const LEVEL1: &str = "翻訳した文のみを出力してください";
    const LEVEL2: &str = "軽く説明した文のみを出力してください";
    const LEVEL3: &str = "説明し、要約した文のみを出力してください";
    const LEVEL4: &str = "詳しく説明した文のみを出力してください";
    const LEVEL5: &str = "例文を用いて説明した文のみを出力してください";

    fn template(prompt: &str, section: &str) -> String {
        format!("英語の`{}`の意味を日本語で{}", section, prompt)
    }

    fn prompt(section: &String, level: usize) -> String {
        match level {
            1 => Self::template(Self::LEVEL1, section),
            2 => Self::template(Self::LEVEL2, section),
            3 => Self::template(Self::LEVEL3, section),
            4 => Self::template(Self::LEVEL4, section),
            5 => Self::template(Self::LEVEL5, section),
            _ => panic!("meaning level is not found"),
        }
    }
}

/// カーソルをブロック(点滅)にする
fn restore_cursor() {
    print!("\x1B[1 q");
}

/// カーソルを棒にする
fn cursor_change() {
    print!("\x1B[5 q");
}

/// カーソルを移動
fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), MoveTo(x, y)).unwrap();
}

/// 文字列を色付きで出力
fn print(str: &str, color: Color) {
    if str == " " && color == Color::Red {
        execute!(stdout(), SetBackgroundColor(color), Print(str), ResetColor).unwrap();
    } else {
        execute!(stdout(), SetForegroundColor(color), Print(str), ResetColor).unwrap();
    }
}

/// 画面をクリア
fn clear() {
    print!("\x1B[2J\x1B[H");
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn input_msg<T: std::str::FromStr>(str: &str, y: u16) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    println!("{}", str);
    move_cursor(str.len() as u16 + 1, y);
    input().trim().parse().expect("parse error")
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
