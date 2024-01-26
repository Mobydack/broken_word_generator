use crate::parser::generate_map_from_preset;
use crate::word_generator::generate;
use crate::dataset_loader::*;

mod dataset_loader {
    use std::env;
    use std::fs;

    pub struct Config {
        path_to_dataset_file: String,
    }

    impl Config {
        pub fn new() -> Self {
            Config {
                path_to_dataset_file: env::var("PATH_TO_FILE").unwrap_or("./dataset/words.txt".to_string()),
            }
        }
    }

    pub fn get_dataset(conf: &Config) -> Vec<String> {
        fs::read_to_string(conf.path_to_dataset_file.clone())
            .unwrap()
            .split('\n')
            .filter(|&line| !line.trim().is_empty())
            .map(|word| word.trim().to_string())
            .collect()
    }
}

mod parser {
    use std::collections::HashMap;

    const START_POSITION_TOKEN: char = '^';
    const END_POSITION_TOKEN: char = '$';

    pub type Model = Box<HashMap<Token, Box<HashMap<Token, u16>>>>;

    #[derive(Debug, Eq, Copy, Clone, Hash)]
    pub enum Token {
        StartPosition,
        EndPosition,
        Symbol(char),
    }

    impl PartialEq for Token {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Token::StartPosition, Token::StartPosition) => true,
                (Token::EndPosition, Token::EndPosition) => true,
                (Token::Symbol(a), Token::Symbol(b)) => a.eq(b),
                _ => false,
            }
        }
    }

    fn get_token(symbol: &char) -> Token {
        match symbol {
            &START_POSITION_TOKEN => Token::StartPosition,
            &END_POSITION_TOKEN => Token::EndPosition,
            _ => Token::Symbol(symbol.clone()),
        }
    }

    pub fn generate_map_from_preset(
        dataset: &Vec<String>,
    ) -> Model {
        let mut result = Box::new(HashMap::new());

        for word in dataset.iter() {
            if word.trim().is_empty() {
                continue;
            }

            let normalized_world = format!("^{}$", word.trim().to_lowercase());
            let mut parent_token: Option<Token> = None;
            let mut chars_iter =  normalized_world.chars();

            while let Some(value) = chars_iter.next() {
                let token = get_token(&value);

                result
                    .entry(token)
                    .or_insert_with(|| Box::new(HashMap::new()));

                if let Some(parent) = parent_token {
                    result
                        .entry(parent)
                        .and_modify(|children| {
                            children
                                .entry(token)
                                .and_modify(|token_count| *token_count += 1)
                                .or_insert(1);
                        });
                }

                parent_token = Some(token);
            };
        };

        result
    }
}

mod word_generator {
    use rand::prelude::*;
    use rand::distributions::WeightedIndex;
    use super::parser::{Model, Token};

    pub fn generate(model: &Model) -> String {
        let mut rng = rand::thread_rng();
        let mut word = String::new();
        let mut curr_token = Token::StartPosition;

        loop {
            match model.get(&curr_token) {
                Some(token_value) => {
                    let mut weights = token_value.iter();
                    let distributions = WeightedIndex::new(weights.clone().map(|(_, value)| value)).unwrap();

                    if let Some((token, _)) = weights.nth(distributions.sample(&mut rng)) {
                        match token {
                            Token::EndPosition => {
                                if word.len() >= 3 {
                                    return word;
                                } else {
                                    continue;
                                }
                            },
                            Token::Symbol(symbol) => {
                                word.push(symbol.clone());

                                if word.len() == 1 {
                                    word = word.to_uppercase();
                                }
                            },
                            _ => (),
                        }

                        curr_token = token.clone();
                    }
                },
                _ => {
                    return word;
                },
            }
        }
    }
}

fn main() {
    let dataset = get_dataset(&Config::new());

    let parser_result = generate_map_from_preset(&dataset);

    for _ in 0..10 {
        println!("{:?}", generate(&parser_result));
    }
}
