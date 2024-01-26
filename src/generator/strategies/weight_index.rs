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

impl Strategy for WeightIndexStrategy {
    fn execute(&self, model: &Model) -> String {
        let mut rng = thread_rng();
        let mut word = String::new();
        let mut curr_token = Token::StartPosition;

        loop {
            match model.transitions.get(&curr_token) {
                Some(transition) => {
                    let mut weights = transition.0.iter();
                    let distributions =
                        WeightedIndex::new(weights.clone().map(|(_, value)| value)).unwrap();

                    if let Some((token, _)) = weights.nth(distributions.sample(&mut rng)) {
                        match token {
                            Token::EndPosition => {
                                if word.len() >= 3 {
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
