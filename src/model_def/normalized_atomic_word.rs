use super::token::{END_POSITION_TOKEN, START_POSITION_TOKEN};

pub struct NormalizedAtomicWord(pub String);

impl NormalizedAtomicWord {
    pub fn new(raw_word: &str) -> Self {
        NormalizedAtomicWord(format!(
            "{}{}{}",
            START_POSITION_TOKEN,
            raw_word.trim().to_lowercase(),
            END_POSITION_TOKEN
        ))
    }
}
