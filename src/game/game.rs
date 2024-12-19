use crossterm::style::Color;

use crate::utils::{clear, move_cursor, print};

use super::{char::Char, config::GameConfig, question::Question, state::State, storage::Storage};

pub struct Game {
    question: Question,
    input: Vec<Char>,
    now_section: String,
    sections: Vec<String>,
    storage: Storage,
    state: State,
    config: GameConfig,
}

impl Game {
    pub fn new(level: usize, meaning_level: usize, number: u8) -> Self {
        let config = GameConfig::new(level, meaning_level, number);
        let question = config.questions()[0].clone();
        let mut game = Self {
            question: question.clone(),
            input: Vec::new(),
            now_section: String::new(),
            sections: question
                .english()
                .split(' ')
                .map(|s| s.to_string())
                .collect(),
            storage: Storage::new(meaning_level),
            state: State::default(),
            config,
        };
        game.change_section();
        game
    }
    pub fn draw(&mut self) {
        clear();
        println!("{}", self.question.english());
        println!("{}", self.question.japanese());
        println!("{}", self.now_section);
        println!("{:?}", self.sections);
        println!("{:?}", self.storage.wrong_sections());
        move_cursor(0, 0);
        self.input.iter().for_each(|c| c.print());
    }
    pub fn set_sections(&mut self) {
        self.sections = self
            .question
            .english()
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
        self.question = self.config.questions()[self.state.count() as usize].clone();
    }
    /// 入力
    pub fn input(&mut self, c: char) {
        // 入力結果が正しいかどうか
        if self
            .question
            .english()
            .chars()
            .nth(self.input.len())
            .unwrap()
            == c
        {
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
        if let Some(count) = self.storage.wrong_sections().get_mut(&wrong_section) {
            *count += 1
        } else {
            // TODO
            // self.storage.wrong_sections().insert(wrong_section, 1);
        }
    }
    /// 入力結果を一文字削除
    pub fn delete(&mut self) {
        self.input.pop();
    }
    /// 次の問題へ
    pub fn next_question(&mut self) {
        self.state.count_up(); // 問題数をカウント
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
        self.input.iter().map(|c| c.c()).collect::<String>() == *self.question.english()
    }
    /// 正回数をカウント
    pub fn correct_count(&mut self) {
        self.state.correct_up();
    }
    /// 入力が終了したか
    pub fn is_input_end(&self) -> bool {
        self.input.len() == self.question.english().len()
    }
    /// ゲームが終了したか
    pub fn is_game_end(&self) -> bool {
        self.state.count() >= self.config.number()
    }
    /// ゲーム終了時の処理
    pub fn quit(&self) {
        clear();
        self.storage.print_inputs();
        println!("level: {}", self.config.level());
        print(
            &format!(
                "correct: {}/{}\n",
                self.state.correct(),
                self.config.number()
            ),
            Color::Green,
        );
        print(
            &self.state.time().elapsed().as_secs_f32().to_string(),
            Color::Green,
        );
        println!();
    }
    pub async fn show_meanings(&mut self) {
        self.storage
            .show_meanings(self.config.meaning_level())
            .await;
    }
}
