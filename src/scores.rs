use std::fs;

use crate::constants::HIGH_SCORE_FILE_PATH;

#[derive(Default)]
pub struct Scores {
    pub score: u32,
    pub high_score: u32,
}

impl Scores {
    pub fn new() -> Self {
        let mut scores = Self::default();
        scores.load_high_score();

        scores
    }

    pub fn check_score_vs_high_score(&mut self) {
        if self.score > self.high_score {
            self.save_high_score();
        }
    }

    fn load_high_score(&mut self) {
        self.high_score = fs::read_to_string(HIGH_SCORE_FILE_PATH)
            .map_or(Ok(0), |i| i.parse::<u32>())
            .unwrap_or(0);
    }

    fn save_high_score(&mut self) {
        self.high_score = self.score;
        fs::write(HIGH_SCORE_FILE_PATH, self.score.to_string()).unwrap();
    }
}