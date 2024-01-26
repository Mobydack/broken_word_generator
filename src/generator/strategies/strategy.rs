use crate::model_def::model::Model;

pub trait Strategy {
    fn execute(&self, model: &Model) -> String;
}
