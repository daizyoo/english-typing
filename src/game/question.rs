#[derive(Clone)]
pub struct Question {
    english: String,
    japanese: String,
}

impl Question {
    pub fn new(english: &'static str, japanese: &'static str) -> Self {
        Self {
            english: english.to_string(),
            japanese: japanese.to_string(),
        }
    }

    pub fn english(&self) -> &String {
        &self.english
    }
    pub fn japanese(&self) -> &String {
        &self.japanese
    }
}
