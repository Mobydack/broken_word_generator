use super::token::Token;
use std::collections::HashMap;

pub struct Transition(pub HashMap<Token, u16>);

impl Transition {
    pub fn new() -> Self {
        Transition(HashMap::new())
    }
}
