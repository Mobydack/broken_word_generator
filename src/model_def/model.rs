use super::{normalized_atomic_word::NormalizedAtomicWord, token::Token, transition::Transition};
use std::collections::HashMap;

pub struct Metadata {
    _len: Vec<u16>,
}

pub struct Model {
    pub metadata: Metadata,
    pub transitions: HashMap<Token, Box<Transition>>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            metadata: Metadata { _len: vec![] },
            transitions: HashMap::new(),
        }
    }

    pub fn apply_preset<'a, I, V>(&mut self, preset_iterator: I)
    where
        I: Iterator<Item = V>,
        V: AsRef<str> + 'a,
    {
        preset_iterator.for_each(|word| {
            let normalized_atomic_word = NormalizedAtomicWord::new(word.as_ref());

            if normalized_atomic_word.0.is_empty() {
                return;
            }

            self.process_word(&normalized_atomic_word);
        });
    }

    fn process_word(&mut self, normalized_atomic_word: &NormalizedAtomicWord) {
        let mut parent_token_option: Option<Token> = None;

        normalized_atomic_word.0.chars().for_each(|character| {
            let token = Token::get_token(&character);

            self.transitions
                .entry(token)
                .or_insert(Box::new(Transition::new()));

            if let Some(parent_token) = parent_token_option {
                self.transitions.entry(parent_token).and_modify(|children| {
                    children
                        .0
                        .entry(token)
                        .and_modify(|token_count| *token_count += 1)
                        .or_insert(1);
                });
            }

            parent_token_option = Some(token);
        });
    }
}
