use serde::{Deserialize, Serialize};

use crate::application::{GenerateUseCase, Type};
use crate::domain::Node;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Generate {}

impl Generate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Type for Generate {
    fn create_use_case(&self, node: Node) -> Box<dyn crate::application::UseCase> {
        Box::new(GenerateUseCase::new(node))
    }
}
