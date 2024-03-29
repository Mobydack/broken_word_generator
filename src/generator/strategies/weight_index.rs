use super::strategy::Strategy;
use crate::model_def::{model::Model, token::Token};
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct WeightIndexStrategy {}

impl WeightIndexStrategy {
    pub fn new() -> Self {
        WeightIndexStrategy {}
    }
}

impl WeightIndexStrategy {

}

impl Strategy for WeightIndexStrategy {
    fn execute(&self, model: &Model) -> String {
        let mut rng = thread_rng();
        let mut word = String::new();

        let mut available_lengths = model.transitions
            .keys()
            .filter(|&token| match token {
                Token::StartPosition(_len) => true,
                _ => false
            })
            .map(|token| match token {
                Token::StartPosition(length) => length,
                _ => &0,
            });
        let length_distributions = WeightedIndex::new(available_lengths.clone()).unwrap();

        let word_length = available_lengths.nth(length_distributions.sample(&mut rng)).unwrap();
        let mut curr_token = Token::StartPosition(*word_length);

        loop {
            match model.transitions.get(&curr_token) {
                Some(transition) => {
                    let mut weights = transition.0.iter();
                    let distributions =
                        WeightedIndex::new(weights.clone().map(|(_, value)| value)).unwrap();

                    if let Some((token, _)) = weights.nth(distributions.sample(&mut rng)) {
                        match token {
                            Token::EndPosition => {
                                if word.len() >= *word_length {
                                    return word;
                                } else {
                                    continue;
                                }
                            }
                            Token::Symbol(symbol) => {
                                word.push(*symbol);

                                if word.len() == 1 {
                                    word = word.to_uppercase();
                                }
                            }
                            _ => (),
                        }

                        curr_token = *token;
                    }
                }
                _ => {
                    return word;
                }
            }
        }
    }
}
