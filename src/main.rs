mod generator;
mod model_def;

use generator::prelude::*;
use model_def::prelude::*;
use std::fs;

fn main() {
    let file_content = fs::read_to_string("./dataset/words.txt").unwrap();
    let mut model = Model::new();
    let strategy = WeightIndexStrategy::new();

    model.apply_preset(file_content.split('\n'));

    println!("Generated word: {:?}", strategy.execute(&model));
}
