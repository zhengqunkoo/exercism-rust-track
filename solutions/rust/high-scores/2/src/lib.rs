#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_scores = self.scores.clone(); // Create a mutable copy of the scores
        top_scores.sort_by(|a, b| b.cmp(a)); // Sort the scores in descending order
        top_scores.into_iter().take(3).collect() // Take the top three scores and collect them into a Vec
    }
}
