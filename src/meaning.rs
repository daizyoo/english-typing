use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Meaning {
    section: String,
    meaning: String,
}

impl Meaning {
    const LEVEL1: &str = "翻訳した文のみを出力してください";
    const LEVEL2: &str = "軽く説明した文のみを出力してください";
    const LEVEL3: &str = "説明し、要約した文のみを出力してください";
    const LEVEL4: &str = "詳しく説明した文のみを出力してください";
    const LEVEL5: &str = "例文を用いて説明した文のみを出力してください";

    pub fn new(section: String, meaning: String) -> Self {
        Self { section, meaning }
    }

    pub fn section(&self) -> &String {
        &self.section
    }
    pub fn meaning(&self) -> &String {
        &self.meaning
    }

    fn template(prompt: &str, section: &str) -> String {
        format!("英語の`{}`の意味を日本語で{}", section, prompt)
    }

    pub fn prompt(section: &str, level: usize) -> String {
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
