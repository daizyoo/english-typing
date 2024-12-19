use std::time::Instant;

pub struct State {
    count: u8,
    correct: u8,
    time: Instant,
}

impl State {
    pub fn count(&self) -> u8 {
        self.count
    }
    pub fn correct(&self) -> u8 {
        self.correct
    }
    pub fn time(&self) -> Instant {
        self.time
    }

    pub(super) fn count_up(&mut self) {
        self.count += 1
    }
    pub(super) fn correct_up(&mut self) {
        self.correct += 1
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
