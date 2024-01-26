mod generator;
mod model_def;

use generator::prelude::*;
use model_def::prelude::*;

fn main() {
    let data_preset = vec![
        "Aaren", "Aarika", "Abagael", "Abagail", "Abbe", "Abbey", "Abbi", "Abbie", "Abby", "Abra",
    ]
    .into_iter();
    let mut model = Model::new();
    let strategy = WeightIndexStrategy::new();

    model.apply_preset(data_preset);

    println!("Generated word: {:?}", strategy.execute(&model));
}
