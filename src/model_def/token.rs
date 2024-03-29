pub const START_POSITION_TOKEN: char = '^';
pub const END_POSITION_TOKEN: char = '$';

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Token {
    StartPosition(usize),
    EndPosition,
    Symbol(char),
}

impl Token {
    pub fn get_token(word: &str, character: &char) -> Token {
        match *character {
            START_POSITION_TOKEN => Token::StartPosition(word.len()),
            END_POSITION_TOKEN => Token::EndPosition,
            _ => Token::Symbol(*character),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, DistString};

    #[test]
    fn should_get_correct_token_for_different_characters() {
        let word = "word";
        let mut samples = vec![('$', Token::EndPosition), ('^', Token::StartPosition(word.len()))];

        Alphanumeric
            .sample_string(&mut rand::thread_rng(), 16)
            .chars()
            .for_each(|character| {
                samples.push((character.clone(), Token::Symbol(character.clone())))
            });

        samples.iter().for_each(|(character, expected_token)| {
            assert_eq!(Token::get_token(word, character), *expected_token)
        });
    }
}
